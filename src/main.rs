use std::sync::{Mutex};

use actix_web::{App, HttpResponse, HttpServer, web, middleware, guard};
use rand::seq::SliceRandom;
use askama::Template;
use actix_web::body::Body;
use actix_web::http::header;

mod commits;

struct Commits {
    messages: Vec<&'static str>,
}

#[derive(Template)]
#[template(path = "dist/index.html")]
struct CommitTemplate<'a> {
    commit: &'a str,
}

async fn plaintext(data: web::Data<Mutex<Commits>>) -> HttpResponse<Body> {
    let data = match data.lock() {
        Ok(commits) => commits,
        Err(e) => panic!("Failed to get commit messages: {}", e)
    };

    return match data.messages.choose(&mut rand::thread_rng()) {
        None => HttpResponse::Ok().content_type("text/plain").body("None"),
        Some(message) => HttpResponse::Ok().content_type("text/plain").body(message.to_owned())
    };
}

async fn html(data: web::Data<Mutex<Commits>>) -> HttpResponse<Body> {
    let data = match data.lock() {
        Ok(commits) => commits,
        Err(e) => panic!("Failed to get commit messages: {}", e)
    };

    let html = CommitTemplate {
        commit: match data.messages.choose(&mut rand::thread_rng()) {
            None => "None",
            Some(message) => message
        }
    }
        .render()
        .unwrap();

    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().body("Healthy\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let commits = web::Data::new(Mutex::new(Commits { messages: commits::get_commits() }));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(commits.clone())
            .service(web::resource("/")
                .route(
                    web::route()
                        .guard(
                            guard::fn_guard(
                                |req| req.headers().get(header::USER_AGENT).unwrap().to_str().unwrap().contains("curl")))
                        .to(plaintext))
                .route(
                    web::route()
                        .to(html)))
            .service(web::resource("/health").route(web::route().to(health)))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
