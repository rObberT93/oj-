use std::cmp::Ordering;
use std::str::FromStr;
use std::vec;
use serde::{Deserialize, Serialize};
use actix_web::{post, get, web::Path, web::Data, web::Query, web::Json, Responder, HttpResponse};
use chrono::{DateTime, FixedOffset};
use std::fs;
use std::collections::HashSet;

use crate::job::{Job, ResultType};
use crate::{JOB_LIST, UESR_LIST, CONTEST_LIST, NewConfig};
use crate::config::Problem;
use crate::user::User;
use crate::err::{Error, ErrorType};

#[derive(Serialize, Deserialize, Clone)]
pub struct Contest {
    pub id: Option<u32>, //可选，如果指定了 ID，则要更新比赛；如果没有指定 ID，则要创建新比赛
    pub name: String, //必选，比赛名称
    pub from: String, //必选，比赛开始时间，时区为 UTC，格式为 %Y-%m-%dT%H:%M:%S%.3fZ
    pub to: String, //必选，比赛结束时间，时区为 UTC，格式为 %Y-%m-%dT%H:%M:%S%.3fZ
    pub problem_ids: Vec<u32>, //必选，一个数组，比赛中所有题目的 ID，不允许出现重复
    pub user_ids: Vec<u32>, //必选，一个数组，比赛中所有用户的 ID，不允许出现重复
    pub submission_limit: u32, //必选，提交次数限制，即每个用户在每个题目上提交次数的最大值，如果不限制，则为 0
}

//可选（默认为 latest），针对同一个用户同一个题目不同提交的评分方式
#[derive(Serialize, Deserialize, Clone)]
pub enum ScoringRule {
    #[serde(rename = "latest")]
    Latest,         //按最后一次提交算分
    #[serde(rename = "highest")]
    Highest,        //按最后一次提交算分
}

//可选，当有多个用户的分数相同时，用于打破平局的规则
#[derive(Serialize, Deserialize, Clone)]
pub enum TieBreaker {
    #[serde(rename = "submission_time")]
    SubmissionTime,     //每个用户每个题目按照 scoring_rule 找到评分所使用的提交，再按每个用户所有题目评分使用的提交时间的最晚时间升序，如果用户所有题目一个提交都没有，则取时间无穷晚
    #[serde(rename = "submission_count")]
    SubmissionCount,    //按总提交数量升序
    #[serde(rename = "user_id")]
    UserId,             //按用户 ID 升序
    None,               //如果不提供此参数，或者即使提供了此参数，也无法打破平局，则平局的用户赋予相同名次，并按照用户 ID 升序排列
}

fn scoring_rule_default() -> ScoringRule { ScoringRule::Latest }

fn tie_breaker_default() -> TieBreaker { TieBreaker::None }

#[derive(Serialize, Deserialize, Clone)]
pub struct RankRule {
    #[serde(default = "scoring_rule_default")]
    pub scoring_rule: ScoringRule,
    #[serde(default = "tie_breaker_default")]
    pub tie_breaker: TieBreaker,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRank {
    pub user: User, //用户
    pub rank: u32, // 排名
    pub scores: Vec<f64>, //有效提交的题目分数
}

struct RankHelper {
    user: User, //id + name
    latest_time: String, //最后一次提交时间
    all_submissions: Vec<Job>, //某个用户在这场比赛中的所有提交的题目
    score: f64, //有效提交的题目的分数之和
    scores: Vec<f64>,
}

#[post("/contests")]
async fn post_contests(body: Json<Contest>, config: Data<NewConfig>) -> impl Responder {
    let mut contest_list = CONTEST_LIST.lock().unwrap();
    let contest = &mut body.into_inner();

    //如果 id 字段存在
    if let Some(id) = contest.id {
        //传入的 id 等于0
        if id == 0 {
            return HttpResponse::BadRequest().json(Error {
                reason: ErrorType::ErrInvalidArgument,
                code: 1,
                message: format!("Invalid contest id."),
            });
        } else {
            //根据 ID 更新比赛
            if let Some(index) = contest_list.iter().position(|c| c.id == Some(id)) {
                //更新比赛出现了重复题目或用户 ID
                match validate_format(&contest, &config.config.problems) {
                    Ok(_) => {},
                    Err(str) => {
                        return HttpResponse::BadRequest().json(Error {
                            reason: ErrorType::ErrInvalidArgument,
                            code: 1,
                            message: str,
                        }); 
                    }
                };
                if !config.config.problems.iter().any(|p| contest.problem_ids.contains(&p.id))
                || !UESR_LIST.lock().unwrap().iter().any(|u| contest.user_ids.contains(&u.id.unwrap())) {
                    //更新比赛出现了不存在的题目或用户
                    return HttpResponse::NotFound().json(Error {
                        reason: ErrorType::ErrNotFound,
                        code: 3,
                        message: format!("Contest {} not found.", contest.id.unwrap()),
                    }); 
                } 
                else {
                    contest_list[index] = contest.clone();
                    let contests: String = serde_json::to_string_pretty(&*contest_list).unwrap();
                    fs::write("./data/contests.json", contests).unwrap();                
                }
            } else {
                //根据 ID 找不到比赛
                return HttpResponse::NotFound().json(Error {
                    reason: ErrorType::ErrNotFound,
                    code: 3,
                    message: format!("Contest {} not found.", contest.id.unwrap()),
                }); 
            }
        } 
    }
    //id 字段不存在
    else {
        //创建新比赛出现了重复的题目或用户
        match validate_format(&contest, &config.config.problems) {
            Ok(_) => {},
            Err(str) => {
                return HttpResponse::BadRequest().json(Error {
                    reason: ErrorType::ErrInvalidArgument,
                    code: 1,
                    message: str,
                }); 
            }
        };
        //创建新比赛出现了不存在的题目或用户
        if !config.config.problems.iter().any(|p| contest.problem_ids.contains(&p.id))
            || !UESR_LIST.lock().unwrap().iter().any(|u| contest.user_ids.contains(&u.id.unwrap()))
        {
            return HttpResponse::NotFound().json(Error {
                reason: ErrorType::ErrNotFound,
                code: 3,
                message: format!("Contest update error"),
            }); 
        } else {
            let max_id = contest_list.iter().map(|c| c.id.unwrap_or(0)).max().unwrap_or(0);
            let new_id = if max_id == 0 { 1 } else { max_id + 1 };
            
            contest.id = Some(new_id);
            contest_list.push(contest.clone());
            let contests: String = serde_json::to_string_pretty(&*contest_list).unwrap();
            fs::write("./data/contests.json", contests).unwrap();
        }
    }
    
    return HttpResponse::Ok().json(contest);
}

fn validate_format(contest: &Contest, problems: &[Problem]) -> Result<(), String> {
    let users = UESR_LIST.lock().unwrap(); 
    // 验证用户ID是否不重复
    let mut user_ids = HashSet::new();
    for user_id in &contest.user_ids {
        if let Some(index) = users.iter().position(|u| u.id == Some(*user_id)) {
            if !user_ids.insert(index) {
                // 用户ID重复，返回错误信息
                return Err(format!("Invalid argument user_id={}", *user_id));
            }
        }
    }
    drop(users);
    // 验证题目ID是否不重复
    let mut problem_ids = HashSet::new();
    for problem_id in &contest.problem_ids {
        if let Some(index) = problems.iter().position(|p| p.id == *problem_id) {
            if !problem_ids.insert(index) {
                // 题目ID重复，返回错误信息
                return Err(format!("Invalid argument problem_id={}", *problem_id));
            }
        }
    }

    Ok(())
}

#[get("/contests")]
async fn get_contests() -> impl Responder {
    //以 JSON 响应返回所有比赛，按照 ID 升序排列
    let mut _contest_list = CONTEST_LIST.lock().unwrap().to_vec().clone();
    _contest_list = CONTEST_LIST.lock().unwrap().to_vec().iter()
        .filter(|x| x.id.unwrap() != 0)
        .cloned().collect();
    _contest_list.sort_by(|a, b| a.id.cmp(&b.id));
    
    HttpResponse::Ok().json(_contest_list)
}

#[get("/contests/{contest_id}")]
async fn get_contest_by_id(contest_id: Path<u32>) -> impl Responder {
    let id = contest_id.into_inner();
    let contest = CONTEST_LIST.lock().unwrap().to_vec().iter().find(|x| x.id.unwrap() == id).cloned();
    match contest {
        None => {
            HttpResponse::NotFound().json(Error{
                code: 3,
                reason: ErrorType::ErrNotFound,
                message: format!("Contest {} not found.", id),
            })
        }
        Some(c) => {
            HttpResponse::Ok().json(c)
        }
    }
}

//针对一个用户一个题目不同提交的评分方式，对一个用户，获取该用户在该比赛中每一个的有效提交
fn get_user_jobs(user: &User, job_list: &Vec<Job>, scoring_rule: &ScoringRule, problems: &Vec<Problem>) -> RankHelper {
    let job_list = job_list.clone();
    let mut user_jobs: Vec<Job> = vec![]; //一场比赛中一个用户提交的所有job
    let mut score: f64 = 0.0; 

    //该用户在这场比赛中的所有提交
    for job in job_list.clone() {
        if job.submission.user_id == user.id.unwrap() {
            user_jobs.push(job.clone());
        }
    }

    //按照题目对用户进行筛选，一个题目下可能有多次提交，筛选出这多次提交中最符合标准的一个
    let mut return_jobs: Vec<Job> = vec![];
    let mut return_scores: Vec<f64> = vec![];

    //对每一个题目
    for problem in problems.iter() { 
        //一道题的所有提交
        let mut submissions: Vec<Job> = user_jobs
            .iter()
            .filter(|job| job.submission.problem_id == problem.id)
            .map(|job| job.clone())
            .collect();

        //记录该用户这道题所有accepted的job
        let mut user_accepted_jobs: Vec<Job> = submissions
            .iter()
            .filter(|job| job.result == ResultType::Accepted)
            .map(|job| job.clone())
            .collect();
        
        //这道题所有用户提交中所有accepted的job
        let all_accepted_jobs: Vec<Job> = job_list
            .iter()
            .filter(|job| job.result == ResultType::Accepted && job.submission.problem_id == problem.id)
            .map(|job| job.clone())
            .collect();

        //根据评分规则对提交进行排序
        match scoring_rule {
            //latest（按最后一次提交算分）
            ScoringRule::Latest => {
                submissions.sort_by(|a, b| {
                    let a_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&a.created_time).unwrap();
                    let b_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&b.created_time).unwrap();
                    b_time.cmp(&a_time)
                });
            }
            //highest（按分数最高的提交中提交时间最早的提交算分）
            ScoringRule::Highest => {
                submissions.sort_by(|a, b| {
                    match b.score.partial_cmp(&a.score) {
                        Some(Ordering::Equal) => {
                            let a_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&a.created_time).unwrap();
                            let b_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&b.created_time).unwrap();        
                            a_time.cmp(&b_time)
                        }
                        other => {
                            other.unwrap_or(Ordering::Equal)
                        }
                    }
                });
            }
        }    
        //竞争得分        
        if problem.misc.dynamic_ranking_ratio.is_some() {
            //计算排行榜时，如果用户有正确的提交，则忽略 scoring_rule，按照用户最后一次正确的提交算分        
            if !user_accepted_jobs.is_empty() {
                user_accepted_jobs.sort_by(|a, b| {
                    let a_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&a.created_time).unwrap();
                    let b_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&b.created_time).unwrap();
                    b_time.cmp(&a_time) //时间降序
                });     
                let mut latest_job = user_accepted_jobs.first().unwrap().clone(); //计算成绩的最后一次提交

                let mut dyn_score: f64 = 100.0 * (1.0 - problem.misc.dynamic_ranking_ratio.unwrap_or(0.0)); //正确性得分
                for case_index in 1..=problem.cases.len() { //遍历每个测试点
                    //选出accept的测试点里面时间最短的
                    let min_time: u32 = all_accepted_jobs.iter().map(|x: &Job| x.cases[case_index].time).min().unwrap();
                    //重新计算成绩
                    dyn_score += problem.cases[case_index - 1].score * problem.misc.dynamic_ranking_ratio.unwrap_or(0.0) * (min_time as f64 / latest_job.cases[case_index].time as f64);
                }
                latest_job.score = dyn_score;
                submissions[0] = latest_job.clone();
            }
        }

        // 取出最符合标准的提交
        if let Some(best_submission) = submissions.first() {
            return_jobs.push((*best_submission).clone());
            score += best_submission.score;
            return_scores.push(best_submission.score);
        } else {
            return_scores.push(0.0);
        }
    }
    
    //筛选出return_jobs中最晚的提交时间
    let latest_time = return_jobs
        .iter()
        .map(|job| chrono::DateTime::from_str(&job.updated_time).unwrap())
        .max()
        .unwrap_or_else(|| chrono::DateTime::parse_from_rfc3339("2000-01-01T00:00:00Z").unwrap())        
        .to_rfc3339();

    RankHelper {
        user: user.clone(),
        latest_time,
        all_submissions: user_jobs, 
        score,
        scores: return_scores,
    }
}

//完成tie_breaker功能
fn compare_users(a: &RankHelper, b: &RankHelper, tie_breaker: &TieBreaker) -> std::cmp::Ordering {
    let mut cmp = b.score.partial_cmp(&a.score).unwrap();
    if cmp == std::cmp::Ordering::Equal {
        match tie_breaker {
            //submission_time（每个用户每个题目按照scoring_rule 找到评分所使用的提交，再按每个用户所有题目评分使用的提交时间的最晚时间升序）如果用户所有题目一个提交都没有，则取时间无穷晚）
            TieBreaker::SubmissionTime => {
                let a_latest_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&a.latest_time).unwrap();
                let b_latest_time: DateTime<FixedOffset> = chrono::DateTime::from_str(&b.latest_time).unwrap();
                cmp = a_latest_time.cmp(&b_latest_time);
            }
            //submission_count（按总提交数量升序）
            TieBreaker::SubmissionCount => {
                cmp = a.all_submissions.len().cmp(&b.all_submissions.len());
            }
            //user_id（按用户 ID 升序）
            TieBreaker::UserId => {
                cmp = a.user.id.unwrap().cmp(&b.user.id.unwrap());
            }
            //如果不提供此参数，或者即使提供了此参数，也无法打破平局，则平局的用户赋予相同名次，并按照用户 ID 升序排列
            TieBreaker::None => { 
                cmp = Ordering::Equal;
            }
        };
    }
    cmp
}

#[get("/contests/{contest_id}/ranklist")]
async fn get_ranklist(cont_id: Path<u32>, rank_rule: Query<RankRule>, config: Data<NewConfig>) -> impl Responder {
    let contest_id = cont_id.into_inner();
    //找到对应的比赛
    let con = CONTEST_LIST.lock().unwrap().iter().find(|x| x.id == Some(contest_id)).cloned();
    let mut rank: Vec<UserRank> = vec![];

    match con {
        None => {
            return HttpResponse::NotFound().json(Error {
                reason: ErrorType::ErrNotFound,
                code: 3,
                message: format!("contest {} not found", contest_id),
            });
        }
        Some(contest) => {
            //比赛中的所有用户
            let mut user_list: Vec<User> = UESR_LIST.lock().unwrap().to_vec().iter().filter(|x| contest.user_ids.contains(&x.id.unwrap())).cloned().collect();

            let scoring_rule = &rank_rule.scoring_rule;
            let tie_breaker = &rank_rule.tie_breaker;
            
            //获取该比赛的所有题目
            let problems: Vec<Problem>;
            if contest.id.unwrap() == 0 {
                problems = config.config.problems.clone();
            } else {
                problems = config.config.problems.iter().filter(|x| contest.problem_ids.contains(&x.id)).cloned().collect();
            }

            //获取该比赛的所有提交
            let job_list: Vec<Job>;
            if contest.id == Some(0) {
                job_list = JOB_LIST.lock().unwrap().clone();
            } else {
                job_list = JOB_LIST.lock().unwrap().iter().filter(|job| job.submission.contest_id == contest.id.unwrap()).cloned().collect();
            }

            //对每个用户进行处理
            user_list.sort_by(|a, b| {
                let a_helper = get_user_jobs(a, &job_list, scoring_rule, &problems);
                let b_helper = get_user_jobs(b, &job_list, scoring_rule, &problems);
                let order: Ordering = compare_users(&a_helper, &b_helper, tie_breaker);
                if let Ordering::Equal = order {
                    a.id.cmp(&b.id)
                } else {
                    order
                }
            });

            rank.push(UserRank { 
                user: user_list[0].clone(), 
                rank: 1,
                scores: get_user_jobs(&user_list[0], &job_list, scoring_rule, &problems).scores,
            });

            for i in 1..user_list.len() {
                let a_helper = get_user_jobs(&user_list[i], &job_list, scoring_rule, &problems);
                let b_helper = get_user_jobs(&user_list[i - 1], &job_list, scoring_rule, &problems);
                if Ordering::Equal == compare_users(&a_helper, &b_helper, tie_breaker) {
                    rank.push(UserRank { user: user_list[i].clone(), rank: rank[i - 1].rank, scores: a_helper.scores});
                }
                else {
                    rank.push(UserRank { user: user_list[i].clone(), rank: (i + 1) as u32, scores: a_helper.scores});
                }
            }
        }
    }

    HttpResponse::Ok().json(rank)
}