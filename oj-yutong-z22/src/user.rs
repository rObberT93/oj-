use serde::{Deserialize, Serialize};
use actix_web::{post, get, web, Responder, HttpResponse};
use std::fs;

use crate::{UESR_LIST, CONTEST_LIST};
use crate::err::{Error, ErrorType};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
}

#[post("/users")]
async fn post_users(user: web::Json<User>) -> impl Responder {
    let mut user_list = UESR_LIST.lock().unwrap();
    let mut user = user.clone();
    let mut name_conflict: bool = false;
    let mut id_exist: bool = false;
    for i in user_list.iter_mut() {
        name_conflict |= user.name == i.name;
        if user.id == i.id {
            i.name = user.name.clone();
            id_exist = true;
            break;
        }
    }
    if name_conflict {
        return HttpResponse::BadRequest().json(Error {
            code: 1,
            reason: ErrorType::ErrInvalidArgument,
            message: format!("User name '{}' already exists.", user.name),
        });
    }
    if user.id.is_none() {
        //新建用户时，如果已有用户，则新用户 ID 为现有用户 ID 最大值加一
        let max_id = user_list.iter().map(|usr| usr.id.unwrap_or(0)).max().unwrap_or(0);
        user.id = Some(max_id + 1);
        user_list.push(user.clone());
        CONTEST_LIST.lock().unwrap()[0].user_ids.push(user.id.unwrap());
        
        let users: String = serde_json::to_string_pretty(&*user_list).unwrap();
        fs::write("./data/users.json", users).unwrap();    
        return HttpResponse::Ok().json(user);
    }
    if !id_exist {
        return HttpResponse::NotFound().json(Error {
            code: 3,
            reason: ErrorType::ErrNotFound,
            message: format!("User {} not found.", user.id.unwrap()),
        });
    }
    let users: String = serde_json::to_string_pretty(&*user_list).unwrap();
    fs::write("./data/users.json", users).unwrap();
    HttpResponse::Ok().json(user)
}

#[get("/users")]
async fn get_users() -> impl Responder {
    //以 JSON 响应返回所有用户，按照 ID 升序排列
    let user_list = UESR_LIST.lock().unwrap().to_vec();
    let mut users = user_list.clone();
    users.sort_by(|a, b| a.id.cmp(&b.id));
    HttpResponse::Ok().json(users)
}
