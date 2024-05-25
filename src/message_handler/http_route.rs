use std::sync::Mutex;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use crate::Game;

pub struct AppState {
    pub game: Mutex<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterData {
    pub user_id: u64,
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinRoomData {
    session_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayData {
    session_id: u64,
    room_id: u64,
    x: u16,
    y: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseResponse {
    rc: u16,
    rd: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountResponse {
    count: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinRoomResponse {
    rc: u16,
    rd: String,
    room_id: u64,
}

pub async fn register(item: web::Json<RegisterData>, data: web::Data<AppState>) -> impl Responder {
    data.game.lock().unwrap().switchboard.insert_new_session(
        crate::switchboard::Session {
            id: item.user_id,
            user_id: item.user_id,
            user_name: item.user_name.clone(),
        }
    );
    let resp = BaseResponse { rc: 0, rd: "Register succeed!".to_string() };
    HttpResponse::Ok().json(resp)
}

pub async fn test_get_list_session(data: web::Data<AppState>) -> impl Responder {
    let count = data.game.lock().unwrap().switchboard.sessions_count();
    let resp = CountResponse { count: count as u16 };
    HttpResponse::Ok().json(resp)
}

pub async fn join_room(item: web::Json<JoinRoomData>, data: web::Data<AppState>) -> impl Responder {
    let resp = JoinRoomResponse {
        rc: 0,
        rd: "".to_string(),
        room_id: 0,
    };
    HttpResponse::Ok().json(resp)
}

pub async fn play(item: web::Json<PlayData>, data: web::Data<AppState>) -> impl Responder {
    let resp = BaseResponse { rc: 0, rd: "".to_string() };
    HttpResponse::Ok().json(resp)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register)))
        .service(web::resource("/test-get-list-session").route(web::get().to(test_get_list_session)))
        .service(web::resource("/join-room").route(web::post().to(join_room)))
        .service(web::resource("/play").route(web::post().to(play)));
}

