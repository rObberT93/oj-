use actix_web::HttpResponse;
use actix_web::{Responder, get};
use serde::{Deserialize, Serialize};
use crate::CONFIG;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProblemType {
    #[default]
    Standard, //标准题，比较时忽略文末空行和行末空格
    Strict, //标准题，严格对比输出和答案
    Spj, //标准题，使用 Special Judge 对比输出
    DynamicRanking, //竞争得分题，使用 standard 模式对比输出，并根据指标竞争得分
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub server: Server,
    pub problems: Vec<Problem>, //必选，记录了所有的题目的数组，数组每个元素是一个字典，每个字典对应一个题目
    pub languages: Vec<Language>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Server {
    #[serde(default = "default_address")]
    bind_address: String, //可选，HTTP 服务器绑定的地址（默认为 127.0.0.1）
    #[serde(default = "default_port")]
    bind_port: i32, //可选，HTTP 服务器绑定的端口（默认为 12345）
}

fn default_address() -> String {
    format!("127.0.0.1")
}

fn default_port() -> i32 {
    12345
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Problem {
    pub id: u32, //必选，每个题目都有唯一的 ID，不保证顺序和连续
    pub name: String, //必选，题目名称
    #[serde(rename = "type")]
    pub typ: ProblemType, //必选，题目类型
    pub misc: Misc, //可选，根据题目类型附加额外的信息，在实现部分提高要求时会涉及
    pub cases: Vec<Case>, //必选，一个记录了所有数据点的数组，数据点按顺序从 1 开始编号，每个数据点是一个字典
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Misc {
    pub packing: Option<Vec<Vec<usize>>>,
    pub special_judge: Option<Vec<String>>,
    pub dynamic_ranking_ratio: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Language { //必选，记录了所有编程语言的数组，数组每个元素是一个字典，每个字典对应一个编程语言
    pub name: String, //必选，编程语言名称
    pub file_name: String, //必选，保存待评测代码的文件名
    pub command: Vec<String>, //必选，一个数组，数组的第一项是所使用的编译器，其余是其命令行参数，其中如果出现了一项为 %INPUT%，
                        // 则要替换其为源代码路径，如果出现了一项为 %OUTPUT%，则要替换其为可执行文件路径
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Case {
    pub score: f64, //必选，该数据点的分数，可以有小数
    pub input_file: String, //必选，该数据点的输入文件
    pub answer_file: String, //必选，该数据点的答案文件
    pub time_limit: u64, //必选，该数据点的时间限制，单位是 us，0 表示不限制
    pub memory_limit: usize, //必选，该数据点的内存限制，单位是字节，0 表示不限制
}

#[get("/config")]
async fn get_config() -> impl Responder {
    let lock = CONFIG.lock().unwrap();
    let config = lock.clone();
    drop(lock);
    return HttpResponse::Ok().json(config);
}