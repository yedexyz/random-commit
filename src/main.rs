use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, web, middleware};
use rand::seq::SliceRandom;

mod commits;

struct Commits {
    messages: Vec<&'static str>,
}

#[get("/")]
async fn get_random_commit(data: web::Data<Mutex<Commits>>) -> HttpResponse {
    let data = data.lock().unwrap();
    HttpResponse::Ok().body(format!("{}\n", data.messages.choose(&mut rand::thread_rng()).unwrap()))
}

#[get("/healthzz")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Healthy\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let commits= web::Data::new(Mutex::new(Commits { messages: commits::get_commits() }));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(commits.clone())
            .service(get_random_commit)
            .service(health)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
