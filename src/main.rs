use std::fs;
use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, web};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Commits {
    messages: Vec<String>,
}

#[get("/")]
async fn index(data: web::Data<Mutex<Commits>>) -> HttpResponse {
    let data = data.lock().unwrap();
    HttpResponse::Ok().body(format!("{}\n", data.messages.choose(&mut rand::thread_rng()).unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting actix-web server");
    let data: String = fs::read_to_string("commits.json").expect("Failed to read commits.json");
    let commits: Commits = serde_json::from_str(&data).expect("Failed to parse commits.json");
    let commit_messages = web::Data::new(Mutex::new(Commits { messages: commits.messages }));

    HttpServer::new(move || {
        App::new()
            .app_data(commit_messages.clone())
            .service(index)
    })
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
