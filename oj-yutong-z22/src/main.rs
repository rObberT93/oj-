use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use env_logger;
use log;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::fs;
use fs::File;
use serde_json;
use chrono::FixedOffset;
use std::io::BufReader;
use actix_cors;
use crossbeam;
use std::thread;

mod job;
mod config;
mod args;
mod err;
mod user;
mod contest;

use job::{Job, post_jobs, get_jobs_by_id, put_jobs, get_jobs, SubmitHelper, evaluate_queue, delete_jobs};
use user::{User, post_users, get_users};
use contest::{Contest, get_contest_by_id, get_contests, get_ranklist, post_contests};
use args::{Args, Parser};
use config::{Config, get_config};

lazy_static! {
    static ref JOB_LIST: Arc<Mutex<Vec<Job>>> = Arc::new(Mutex::new(Vec::new())); //测评任务
    static ref UESR_LIST: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![]));
    static ref CONTEST_LIST: Arc<Mutex<Vec<Contest>>> = Arc::new(Mutex::new(vec![]));
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::default()));
}

#[derive(Clone)]
pub struct NewConfig {
    pub config : Config,
    pub sender : Option<crossbeam::channel::Sender<SubmitHelper>>
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    log::info!(target: "greet_handler", "Greeting {}", name);
    format!("Hello {name}!")
}

// DO NOT REMOVE: used in automatic testing
#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit() -> impl Responder {
    log::info!("Shutdown as requested");
    std::process::exit(0);
    format!("Exited")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let config_path = args.config;
    let is_flush = args.flush_data;
    *CONFIG.lock().unwrap() = serde_json::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();   

    //持久化存储读取数据
    if is_flush {
        UESR_LIST.lock().unwrap().push(User { 
            id: Some(0), 
            name: format!("root"),
        });
        CONTEST_LIST.lock().unwrap().push(
            Contest {
                id: Some(0),
                name: "".to_string(),
                from: {
                    let time: chrono::DateTime<FixedOffset> = chrono::DateTime::default();
                    time.to_string()
                },
                to: {
                    chrono::NaiveDateTime::MAX.to_string()
                },
                problem_ids: vec![],
                user_ids: vec![0],
                submission_limit: 0,
            }
        );
    } else {
        *JOB_LIST.lock().unwrap() = serde_json::from_reader(BufReader::new(File::open("./data/jobs.json").unwrap())).unwrap();
        *UESR_LIST.lock().unwrap() = serde_json::from_reader(BufReader::new(File::open("./data/users.json").unwrap())).unwrap();
        *CONTEST_LIST.lock().unwrap() = serde_json::from_reader(BufReader::new(File::open("./data/contests.json").unwrap())).unwrap();    
    }

    let config = CONFIG.lock().unwrap().clone();

    let mut new_config : NewConfig = NewConfig {
        config : config.clone(),
        sender : None
    };

    let (_sender, receiver) = crossbeam::channel::unbounded::<SubmitHelper>();

    //多线程同时测评
    new_config.sender = Some(_sender.clone());
    for _ in 0..8 {
        let cloned_receiver = receiver.clone();
        thread::spawn(move ||{
            evaluate_queue(cloned_receiver);
        });
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new( move || {
        App::new()
            .wrap(Logger::default())
            .wrap(actix_cors::Cors::permissive()) //cros头部 前端访问
            .app_data(web::Data::new(new_config.clone()))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(get_config)
            .service(post_jobs)
            .service(get_jobs)
            .service(get_jobs_by_id)
            .service(put_jobs)
            .service(post_users)
            .service(get_users)
            .service(get_ranklist)
            .service(post_contests)
            .service(get_contest_by_id)
            .service(get_contests)
            .service(exit)
            .service(delete_jobs)
    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}