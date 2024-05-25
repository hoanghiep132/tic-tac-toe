use std::sync::Mutex;
use actix_web::{App, HttpServer, Responder, web};

use crate::switchboard::Switchboard;

mod switchboard;
mod conf;
mod message_handler;

use message_handler::http_route::config;
use crate::message_handler::http_route::AppState;

#[derive(Debug)]
pub struct Game {
    pub switchboard: Switchboard,
    pub fir_interval: chrono::Duration,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let cfg = conf::SwitchboardConfig {
        max_sessions_per_agent: 100,
        max_agents: None,
    };
    let game = Game {
        switchboard: Switchboard::new(cfg),
        fir_interval: chrono::Duration::seconds(1),
    };

    let app_state = web::Data::new(AppState {
        game: Mutex::new(game),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(config)
    }).bind("127.0.0.1:8080")?
        .run()
        .await
}