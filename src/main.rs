use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, web, middleware::Logger};
use rand::seq::SliceRandom;

mod commits;

struct Commits {
    messages: Vec<&'static str>,
}

#[get("/")]
async fn index(data: web::Data<Mutex<Commits>>) -> HttpResponse {
    let data = data.lock().unwrap();
    HttpResponse::Ok().body(format!("{}\n", data.messages.choose(&mut rand::thread_rng()).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let commits= web::Data::new(Mutex::new(Commits { messages: commits::get_commits() }));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("Status: %s"))
            .app_data(commits.clone())
            .service(index)
    })
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
