use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use actix_web::{post, get, put, web, Responder, HttpResponse, Result, delete};
use std::process::Stdio;
use std::vec;
use std::{fs, process::Command, time::Duration};
use std::io::Read;
use std::fs::create_dir;
use wait_timeout::ChildExt;
use std::str::FromStr;
use crossbeam;
use libc::{self, c_int, rusage, getrusage, RUSAGE_SELF};
use std::mem;

use crate::{JOB_LIST, UESR_LIST, CONTEST_LIST, NewConfig};
use crate::config::{Config, Problem, ProblemType, Case, Language};
use crate::err::{Error, ErrorType};

pub const TIME_FMT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PostJobs {
    pub source_code: String,
    pub language: String,
    pub user_id: u32,
    pub contest_id: u32,
    pub problem_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GetJobs {
    user_id: Option<String>,
    user_name: Option<String>,
    contest_id: Option<String>,
    problem_id: Option<String>,
    language: Option<String>,
    from: Option<String>,
    to: Option<String>,
    state: Option<String>,
    result: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CaseResult {
    pub id: u32,
    pub result: ResultType,
    pub time: u32,
    pub memory: usize,
    pub info: String,
}

impl CaseResult {
    fn default(id: u32) -> CaseResult {
        CaseResult {
            id,
            result: ResultType::Waiting,
            time: 0,
            memory: 0,
            info: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub enum State {
    #[default]
    Queueing,
    Running,
    Finished,
    Canceled,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Job {
    pub id: u32,
    pub created_time: String,
    pub updated_time: String,
    pub submission: PostJobs,
    pub state: State,
    pub result: ResultType,
    pub score: f64,
    pub cases: Vec<CaseResult>,
}

impl Job {
    pub fn default(id: u32, post: &PostJobs) -> Job {
        Job {
            id,
            created_time: Utc::now().format(TIME_FMT).to_string(),
            updated_time: Utc::now().format(TIME_FMT).to_string(),
            submission: post.clone(),
            state: State::Queueing,
            result: ResultType::Waiting,
            score: 0.0,
            cases: vec![],
        }
    }

    fn get_update(&mut self) {
        self.updated_time = Utc::now().format(TIME_FMT).to_string();
        self.state = State::Running;
    }

    fn get_finish(&mut self) {
        self.updated_time = Utc::now().format(TIME_FMT).to_string();
        self.state = State::Finished;
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub enum ResultType {
    #[default]
    Waiting,
    Running,
    Accepted,
    #[serde(rename = "Compilation Error")]
    CompilationError,
    #[serde(rename = "Compilation Success")]
    CompilationSuccess,
    #[serde(rename = "Wrong Answer")]
    WrongAnswer,
    #[serde(rename = "Runtime Error")]
    RuntimeError,
    #[serde(rename = "Time Limit Exceeded")]
    TimeLimitExceeded,
    #[serde(rename = "Memory Limit Exceeded")]
    MemoryLimitExceeded,
    #[serde(rename = "System Error")]
    SystemError,
    #[serde(rename = "SPJ Error")]
    SPJError,
    Skipped,
}

pub struct SubmitHelper {
    language: Language,
    problem: Problem,
    index: usize,
}

fn check_valid(config: &Config, body: &PostJobs) -> Result<(), Error> {
    let current_language = config.languages.iter().find(|x| &x.name == &body.language).cloned();
    let current_problem = config.problems.iter().find(|x| &x.id == &body.problem_id).cloned();
    let lock = CONTEST_LIST.lock().unwrap();
    let contest_list = lock.clone();
    drop(lock);
    let contest = contest_list.iter().find(|x| x.id.unwrap() == body.contest_id);

    //编程语言或题目ID是否存在
    if current_language.is_none() || current_problem.is_none() {
        return Err(Error {
            code: 3,
            reason: ErrorType::ErrNotFound,
            message: format!("HTTP 404 Not Found"),
        });
    }

    //用户ID是否存在
    let lock = UESR_LIST.lock().unwrap();
    if !lock.iter().any(|x| &x.id.unwrap() == &body.user_id) {
        return Err(Error {
            code: 3,
            reason: ErrorType::ErrNotFound,
            message: format!("HTTP 404 Not Found"),
        });
    }
    drop(lock);
    //比赛ID是否存在，只对比赛ID不为0时检查
    if body.contest_id != 0 {
        if contest.is_none() { 
            return Err(Error {
                code: 3,
                reason: ErrorType::ErrNotFound,
                message: format!("HTTP 404 Not Found"),
            });
        }
        let contest = contest.unwrap();
        let from_time: DateTime<Utc> = chrono::DateTime::from_str(&contest.from).unwrap();
        let to_time: DateTime<Utc> = chrono::DateTime::from_str(&contest.to).unwrap();
        //检查用户ID是否在此比赛中
        let user_ids_contains = contest.user_ids.contains(&body.user_id);
        //检查题目ID是否在此比赛中
        let problem_ids_contains = contest.problem_ids.contains(&body.problem_id);
        //提交评测任务时间是否在比赛进行时间范围内
        let is_valid_time_range = Utc::now() > from_time && Utc::now() < to_time;

        if !(user_ids_contains && problem_ids_contains && is_valid_time_range) {
            return Err(Error {
                reason: ErrorType::ErrInvalidArgument,
                code: 1,
                message: format!("HTTP 400 Bad Request"),
            });
        }
        //用户该题目的提交次数是否达到上限
        let lock = JOB_LIST.lock().unwrap();
        let job_list = lock.clone();
        drop(lock);
        let mut counter = 0;
        for j in job_list {
            if j.submission.problem_id == body.problem_id {
                counter += 1;
            }
        }
        if counter >= contest.submission_limit {
            return Err(Error {
                reason: ErrorType::ErrRateLimit,
                code: 4,
                message: format!("HTTP 400 Bad Request"),
            });
        }
    }
    Ok(())
}

#[post("/jobs")]
async fn post_jobs(body: web::Json<PostJobs>, config: web::Data<NewConfig>) -> impl Responder {
    //检查请求的合法性
    match check_valid(&config.config, &body) {
        Err(err) => {
            return err.response();
        }
        Ok(()) => {
            let current_language = config.config.languages.iter().find(|x| &x.name == &body.language).cloned().unwrap();
            let current_problem = config.config.problems.iter().find(|x| &x.id == &body.problem_id).cloned().unwrap();
            let mut lock = JOB_LIST.lock().unwrap();
            let id = lock.len();
            let mut job = Job::default(id as u32, &body.0);

            for i in 0..=current_problem.clone().cases.len() {
                job.cases.push(CaseResult::default(i as u32));
            }
            //把job压入joblist
            lock.push(job.clone());

            //将job写入json文件
            let jobs: String = serde_json::to_string_pretty(&*lock).unwrap();
            match fs::write("./data/jobs.json", jobs) {
                Ok(_) => {}, 
                Err(_) => {
                    // 写入失败
                    return HttpResponse::InternalServerError().json(Error {
                        code : 6,
                        reason : ErrorType::ErrInternal,
                        message : format!("HTTP 500 Internal Server Error"),
                    });
                }
            };
            drop(lock);
        
            let submit_helper = SubmitHelper {
                problem: current_problem,
                language: current_language,
                index: id as usize,
            };
            let sender = config.sender.clone();
        
            //非阻塞测评
            match sender.unwrap().send(submit_helper) {
                Err(_) => {
                    return HttpResponse::InternalServerError().json(Error {
                        code : 6,
                        reason : ErrorType::ErrInternal,
                        message : format!("HTTP 500 Internal Server Error"),
                    });
                }
                Ok(_) => {}
            }
            return HttpResponse::Ok().json(job.clone());
        }
    }
}

pub fn evaluate_queue(receiver: crossbeam::channel::Receiver<SubmitHelper>) {
    loop {
        let r = receiver.try_recv();
        match r {
            Err(_) => {}
            Ok(para) => {
                let lock = JOB_LIST.lock().unwrap();
                let state = lock[para.index].state.clone();
                drop(lock);
                if state == State::Queueing {
                    run_job(&para.problem, &para.language, para.index);
                }
            }
        }
    } 
}

//及时释放lock 防止死锁
pub fn run_job(problem: &Problem, language: &Language, index: usize) {
    let mut lock = JOB_LIST.lock().unwrap();
    lock[index].state = State::Running;
    lock[index].result = ResultType::Running;
    let submission = lock[index].submission.clone();
    let len = lock.len();
    drop(lock);
    //创建临时测评目录
    let temp_dir = format!("./TMPDIR{}", len);

    //创建文件夹前检查 若这个文件夹已经存在则删除
    if let Ok(metadata) = fs::metadata(&temp_dir) {
        if metadata.is_dir() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
    }
    create_dir(&temp_dir).unwrap();

    //创建源代码文件
    let src_file = format!("{}/{}", temp_dir, language.file_name);
    fs::File::create(&src_file).unwrap();
    fs::write(&src_file, &submission.source_code).unwrap();

    //创建输出文件路径
    let out_path: String = format!("{}/job_{}", temp_dir, submission.user_id);

    //替换
    let mut command = language.command.clone();
    let input_index = command.iter().position(|x| x == "%INPUT%").unwrap(); //%INPUT% 在 current_language.command 列表中的索引位置
    let output_index = command.iter().position(|x| x == "%OUTPUT%").unwrap(); //%OUTPUT% 在 current_language.command 列表中的索引位置
    
    command[input_index] = src_file.clone();
    command[output_index] = out_path.clone();
    
    //编译
    let compile_time = std::time::Instant::now();
    let compile_job = Command::new(&command[0]).args(&command[1..]).status().unwrap();
    //更新编译用时
    let mut lock = JOB_LIST.lock().unwrap();
    lock[index].cases[0].time = compile_time.elapsed().as_micros() as u32;
    drop(lock);
    //如果编译以非0码退出
    if compile_job.code() != Some(0) {
        let mut lock = JOB_LIST.lock().unwrap();
        lock[index].result = ResultType::CompilationError;
        lock[index].cases[0].result = ResultType::CompilationError;
        lock[index].get_finish();
        drop(lock);
    } else {
        //编译成功
        let mut lock = JOB_LIST.lock().unwrap();
        lock[index].cases[0].result = ResultType::CompilationSuccess;
        lock[index].get_update();
        let packs: Vec<Vec<usize>>;
        drop(lock);
        match problem.misc.packing.clone() {
            None => packs = vec![(1..=problem.cases.len()).collect()],
            Some(p) => packs = p
        };
        //打包测试
        for pack in packs {
            let mut pack_passed = true;
            let mut pack_score = 0.0;
            for i in pack {
                let mut _case_result = CaseResult::default(i as u32);
                let now = std::time::Instant::now();
                _case_result = evaluate_cases(&problem, i, &out_path);
                if pack_passed {
                    match _case_result.result {
                        ResultType::Accepted => {
                            //如果没有字段 
                            pack_score += problem.cases[i - 1].score * (1.0 - problem.misc.dynamic_ranking_ratio.unwrap_or(0.0));
                        }
                        _ => {
                            //job_result 为第一个出现错误的点的状态
                            let mut lock = JOB_LIST.lock().unwrap();
                            if lock[index].result == ResultType::Running {
                                lock[index].result = _case_result.result.clone();
                            }
                            drop(lock);
                            pack_passed = false;
                            pack_score = 0.0;
                        }
                    };
                } else {
                    //一旦某个点错误，则该组剩余的测试点不再进行评测，结果标记为 Skipped
                    _case_result.result = ResultType::Skipped;
                }
                _case_result.time = now.elapsed().as_micros() as u32; 
                let mut lock = JOB_LIST.lock().unwrap();
                lock[index].cases[i] = _case_result;
                lock[index].get_update();
                drop(lock);
            }
            let mut lock = JOB_LIST.lock().unwrap();
            lock[index].score += pack_score;
            lock[index].get_update();
            drop(lock);
        }
    }
    fs::remove_dir_all(&temp_dir).unwrap();
    //所有数据点评测结果都是 Accepted job_result 为 Accepted
    let mut lock = JOB_LIST.lock().unwrap();
    if lock[index].cases.iter().skip(1).all(|x| x.result == ResultType::Accepted) {
        lock[index].result = ResultType::Accepted;
    }
    lock[index].get_finish();

    let jobs: String = serde_json::to_string_pretty(&*lock).unwrap();
    fs::write("./data/jobs.json", jobs).unwrap();
    drop(lock);
}

fn get_max_memory_usage() -> Option<usize> {
    let mut usage: rusage = unsafe { mem::zeroed() };
    let ret: c_int;

    unsafe {
        //获取当前进程的资源使用信息
        ret = getrusage(RUSAGE_SELF, &mut usage);
    }
    //检查返回值
    if ret == 0 {  //调用成功
        Some(usage.ru_maxrss as usize * 1024) // convert to bytes
    } else { //调用失败
        None
    }
}

fn evaluate_cases(problem: &Problem, case_index: usize, out_path: &String) -> CaseResult {
    let case = &problem.cases[case_index - 1];
    let mut case_result = CaseResult::default(case_index as u32);
    let in_file = fs::File::open(&case.input_file).unwrap();
    //获取当前评测点的内存使用量
    let mut max_memory_usage: Option<usize> = None;
    
    let mut run_case = Command::new(&out_path)
        .stdin(Stdio::from(in_file))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("limitted");
    
    //更新内存使用量
    if let Some(usage) = get_max_memory_usage() {
        eprint!("{usage}");
        if max_memory_usage.is_none() || usage > max_memory_usage.unwrap() {
            max_memory_usage = Some(usage);
        }
    }

    //如果内存限制不为0，超出内存直接报错
    if let Some(usage) = max_memory_usage {
        let memory_limit = case.memory_limit;
        if usage > memory_limit && memory_limit != 0 {
            case_result.memory = usage;
            case_result.result = ResultType::MemoryLimitExceeded;
            return case_result;
        }
    }

    let time_limit = Duration::from_micros(case.time_limit as u64);

    match run_case.wait_timeout(time_limit).unwrap() {
        None => {
            run_case.kill().unwrap();
            case_result.result = ResultType::TimeLimitExceeded;
        }
        Some(status) => {
            if status.success() {
                let out = run_case.stdout;
                let mut out_str = String::new();
                out.unwrap().read_to_string(&mut out_str).unwrap();
                let ans_str = fs::read_to_string(&case.answer_file).unwrap();

                match &problem.typ {
                    // 忽略文末空行和行末空格
                    ProblemType::Standard => {
                        let out_str_vec: Vec<String> = out_str.trim().split("\n").map(|s| s.trim().to_string()).collect();
                        let ans_str_vec: Vec<String> = ans_str.trim().split("\n").map(|s| s.trim().to_string()).collect();
                        if out_str_vec == ans_str_vec {
                            case_result.result = ResultType::Accepted;
                        } else {
                            case_result.result = ResultType::WrongAnswer;
                        }
                    }
                    ProblemType::Strict => {
                        if out_str == ans_str {
                            case_result.result = ResultType::Accepted;
                        } else {
                            case_result.result = ResultType::WrongAnswer;
                        }
                    }
                    ProblemType::Spj => {
                        let spj_res = spj_judge(problem, case, &out_str);
                        case_result.result = spj_res.0;
                        case_result.info = spj_res.1;
                    }
                    ProblemType::DynamicRanking => {
                        //输出和答案的比较方式与 standard 相同
                        let out_str_vec: Vec<String> = out_str.trim().split("\n").map(|s| s.trim().to_string()).collect();
                        let ans_str_vec: Vec<String> = ans_str.trim().split("\n").map(|s| s.trim().to_string()).collect();
                        if out_str_vec == ans_str_vec {
                            case_result.result = ResultType::Accepted;
                        } else {
                            case_result.result = ResultType::WrongAnswer;
                        }
                    }
                };
            }
            // 运行时错误，程序异常退出
            else {
                case_result.result = ResultType::RuntimeError;
            }
        }
    }
    case_result.memory = max_memory_usage.unwrap_or(0);
    case_result
}

//通过外部程序将用户程序输出与标准答案进行对比
fn spj_judge(problem: &Problem, case: &Case, out_str: &String) -> (ResultType, String) {
    let case_result: ResultType;
    let mut spj_info: String = String::new();
    let mut spj_command = problem.misc.special_judge.clone().unwrap();
    //创建输出文件路径
    let lock = JOB_LIST.lock().unwrap();
    let len = lock.len();
    drop(lock);
    let out_path = format!("./TMPDIR{}/spj_output", len);
    fs::File::create(&out_path).expect("fail to create directory");
    fs::write(&out_path, &out_str).unwrap();

    let out_index = spj_command.iter().position(|x| x == "%OUTPUT%").unwrap(); 
    let ans_index = spj_command.iter().position(|x| x == "%ANSWER%").unwrap();
    spj_command[out_index] = out_path.clone();
    spj_command[ans_index] = case.answer_file.clone();

    let run_spj = Command::new(&spj_command[0])
        .args(&spj_command[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to excute SPJ program!");

    if run_spj.status.success() {
        let spj_result = String::from_utf8_lossy(&run_spj.stdout);
        let lines: Vec<&str> = spj_result.lines().collect();
        if let Some(result) = lines.get(0) {
            if let Ok(parsed_result) = serde_json::from_str::<ResultType>(&format!("\"{}\"", result)) {
                case_result = parsed_result;

                if let Some(info) = lines.get(1) {
                    spj_info = info.to_string();
                }
            } else {
                case_result = ResultType::SPJError;
            }
        } else {
            case_result = ResultType::SPJError;
        }
    } else {
        case_result = ResultType::SPJError;
    }
    (case_result, spj_info)
}

#[get("/jobs")]
async fn get_jobs(query: web::Query<GetJobs>) -> impl Responder {
    let mut available_jobs: Vec<Job> = vec![];
    let user_list = UESR_LIST.lock().unwrap();
    let lock = JOB_LIST.lock().unwrap();
    let job_list = lock.clone();
    drop(lock);
    for job in &job_list {
        //按照用户 ID 进行筛选
        if let Some(arg) = &query.user_id {
            if let Ok(user_id) = arg.parse::<u32>() {
                if user_id != job.submission.user_id {
                    continue;
                }
            } else {
                return HttpResponse::BadRequest().json(Error {
                    code: 1,
                    reason: ErrorType::ErrInvalidArgument,
                    message: format!("Invalid argument user_id={}", arg),
                }); 
            }
        }
        //按照用户名进行筛选
        if let Some(arg) = &query.user_name {
            if arg != &(user_list[job.submission.user_id as usize]).name {
                continue;
            }
        }
        //按照比赛 ID 进行筛选
        if let Some(arg) = &query.contest_id {
            if let Ok(contest_id) = arg.parse::<u32>() {
                if contest_id != job.submission.contest_id {
                    continue;
                }
            } else {
                return HttpResponse::BadRequest().json(Error {
                    code: 1,
                    reason: ErrorType::ErrInvalidArgument,
                    message: format!("Invalid argument contest_id={}", arg),
                }); 
            }
        }
        //按照题目 ID 进行筛选
        if let Some(arg) = &query.problem_id {
            if let Ok(problem_id) = arg.parse::<u32>() {
                if problem_id != job.submission.problem_id {
                    continue;
                }
            } else {
                return HttpResponse::BadRequest().json(Error {
                    code: 1,
                    reason: ErrorType::ErrInvalidArgument,
                    message: format!("Invalid argument problem_id={}", arg),
                }); 
            }
        }
        //按照编程语言进行筛选
        if let Some(arg) = &query.language {
            if arg != &job.submission.language {
                continue;
            }
        }
        //筛选出创建时间不早于该参数的评测任务
        if let Some(arg) = &query.from {
            let from_time = DateTime::parse_from_str(&arg, TIME_FMT);
            match from_time {
                Ok(parsed_time) => {
                    let actual_time = DateTime::parse_from_str(&job.created_time, TIME_FMT).unwrap();
                    if parsed_time >= actual_time {
                        continue;
                    }
                }
                Err(_) => {
                    return HttpResponse::BadRequest().json(Error {
                        code: 1,
                        reason: ErrorType::ErrInvalidArgument,
                        message: format!("Invalid argument from={}", arg),
                    });
                }
            }
        }
        //筛选出创建时间不晚于该参数的评测任务
        if let Some(arg) = &query.to {
            let to_time = DateTime::parse_from_str(&arg, TIME_FMT);
            match to_time {
                Ok(parsed_time) => {
                    let actual_time = DateTime::parse_from_str(&job.created_time, TIME_FMT).unwrap();
                    if parsed_time <= actual_time {
                        continue;
                    }
                }
                Err(_) => {
                    return HttpResponse::BadRequest().json(Error {
                        code: 1,
                        reason: ErrorType::ErrInvalidArgument,
                        message: format!("Invalid argument to={}", arg),
                    });
                }
            }
        }
        //按照评测任务当前状态筛选
        if let Some(arg) = &query.state {
            let parsed_state = match arg.as_str() {
                "Queueing" => State::Queueing,
                "Running" => State::Running,
                "Finished" => State::Finished,
                "Canceled" => State::Canceled,
                _ => {
                    return HttpResponse::BadRequest().json(Error {
                        code: 1,
                        reason: ErrorType::ErrInvalidArgument,
                        message: format!("Invalid argument state={}", arg),
                    });
                }
            };
    
            if parsed_state != job.state {
                continue;
            }
        }
        //按照评测任务当前结果筛选
        if let Some(arg) = &query.result {
            let parsed_result = match arg.as_str() {
                "Waiting" => ResultType::Waiting,
                "Running" => ResultType::Running,
                "Accepted" => ResultType::Accepted,
                "Compilation Error" => ResultType::CompilationError,
                "Compilation Success" => ResultType::CompilationSuccess,
                "Wrong Answer" => ResultType::WrongAnswer,
                "Runtime Error" => ResultType::RuntimeError,
                "Time Limit Exceeded" => ResultType::TimeLimitExceeded,
                "Memory Limit Exceeded" => ResultType::MemoryLimitExceeded,
                "System Error" => ResultType::SystemError,
                "SPJ Error" => ResultType::SPJError,
                "Skipped" => ResultType::Skipped,
                _ => {
                    return HttpResponse::BadRequest().json(Error {
                        code: 1,
                        reason: ErrorType::ErrInvalidArgument,
                        message: format!("Invalid argument result={}", arg),
                    });
                }
            };
    
            if parsed_result != job.result {
                continue;
            }
        }
        available_jobs.push(job.clone());
    }
    available_jobs.sort_by(|a, b| a.created_time.cmp(&b.created_time));
    HttpResponse::Ok().json(available_jobs)
}

#[get("/jobs/{jobId}")]
async fn get_jobs_by_id(jobid: web::Path<u32>) -> impl Responder {
    let lock = JOB_LIST.lock().unwrap();
    let res = lock.iter()
        .rfind(|job| job.id == *jobid)
        .cloned();
    drop(lock);
    if let Some(job) = res {
        HttpResponse::Ok().json(job)
    } else {
        HttpResponse::NotFound().json(Error{
            code: 3,
            reason: ErrorType::ErrNotFound,
            message: format!("Job {} not found.", *jobid),
        })
    }
}

#[put("/jobs/{jobId}")]
async fn put_jobs(jobid: web::Path<u32>, config: web::Data<NewConfig>) -> impl Responder {
    let lock = JOB_LIST.lock().unwrap();
    let id = jobid.into_inner();
    for i in 0..lock.len() {
        if lock[i].id == id {
            if lock[i].state != State::Finished {
                return HttpResponse::BadRequest().json( Error {
                    code : 2,
                    reason : ErrorType::ERRINVALIDSTATE,
                    message : format!("Job {} not finished.", id),
                });
            }
            let submission = lock[i].submission.clone();
            drop(lock);
            match check_valid(&config.config, &submission) {
                Ok(()) => {},
                Err(err) => {
                    return err.response();
                }
            }
            let current_language = config.config.languages.iter().find(|x| &x.name == &submission.language).cloned().unwrap();
            let current_problem = config.config.problems.iter().find(|x| &x.id == &submission.problem_id).cloned().unwrap();

            let mut lock = JOB_LIST.lock().unwrap();
            lock[i] = Job::default(id, &submission);
            for j in 0..=current_problem.cases.len() {
                lock[i].cases.push(CaseResult::default(j as u32));
            }
            drop(lock);
            let submit_helper = SubmitHelper {
                problem: current_problem,
                language: current_language,
                index: i as usize,
            };
        
            //非阻塞对JOB_LIST逐个进行测评
            let sender = config.sender.clone();
            match sender.unwrap().send(submit_helper) {
                Err(_) => {
                    return HttpResponse::InternalServerError().json(Error {
                        code: 6,
                        reason: ErrorType::ErrInternal,
                        message: format!("Channel closed."),
                    });
                }
                Ok(_) => {}
            }  
            let lock = JOB_LIST.lock().unwrap();
            let return_job = lock[i].clone();
            drop(lock);
            return HttpResponse::Ok().json(return_job);
        }
    }
    HttpResponse::NotFound().json(Error {
        code: 3,
        reason: ErrorType::ErrNotFound,
        message: format!("Job {} not found.", id),
    })
}

#[delete("/jobs/{jobid}")]
pub async fn delete_jobs(jobid: web::Path<usize>) -> impl Responder {
    let job_id = jobid.into_inner();
    let lock = JOB_LIST.lock().unwrap();
    let job_list = lock.clone();
    drop(lock);

    for job in job_list {
        if job.id as usize == job_id {
            if job.state != State::Queueing {
                return HttpResponse::BadRequest().json(Error {
                    code : 2,
                    reason : ErrorType::ERRINVALIDSTATE,
                    message : format!("Job {} not queuing.", job_id),
                });
            } else {
                let mut lock = JOB_LIST.lock().unwrap();
                lock[job_id].state = State::Canceled;
                lock[job_id].result = ResultType::Skipped;

                let jobs: String = serde_json::to_string_pretty(&*lock).unwrap();
                fs::write("./data/jobs.json", jobs).unwrap();
    
                drop(lock);
                
                return HttpResponse::Ok().finish();//Set an empty body
            }
        }
    }
    HttpResponse::NotFound().json(Error {
        code: 3,
        reason: ErrorType::ErrNotFound,
        message: format!("Job {} not found.", job_id),
    })
}