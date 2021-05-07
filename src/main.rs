use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, web, middleware, HttpRequest};
use rand::seq::SliceRandom;
use askama::Template;
use actix_web::http::{HeaderValue, header};
use actix_web::body::Body;
use actix_web::http::header::ToStrError;

mod commits;

struct Commits {
    messages: Vec<&'static str>,
}

#[derive(Template)]
#[template(path = "dist/index.html")]
struct CommitTemplate<'a> {
    commit: &'a str,
}

fn get_user_agent<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("User-Agent")?.to_str().ok()
}

#[get("/")]
async fn get_random_commit(data: web::Data<Mutex<Commits>>, req: HttpRequest) -> HttpResponse<Body> {
    let data = data.lock().unwrap();

    let user_agent = req
        .headers()
        .get(header::USER_AGENT)
        .map(|hdr| hdr.to_str());

    match user_agent {
        Some(Ok(user_agent)) => {
            if user_agent.contains("curl") {
                HttpResponse::Ok().content_type("text/plain").body(format!("{}\n", data.messages.choose(&mut rand::thread_rng()).unwrap()))
            } else {
                let s = CommitTemplate {
                    commit: data.messages.choose(&mut rand::thread_rng()).unwrap()
                }
                    .render()
                    .unwrap();

                CommitTemplate {
                    commit: data.messages.choose(&mut rand::thread_rng()).unwrap()
                };

                HttpResponse::Ok().content_type("text/html").body(s)
            }
        }
        Some(Err(_)) => {
            let s = CommitTemplate {
                commit: data.messages.choose(&mut rand::thread_rng()).unwrap()
            }
                .render()
                .unwrap();

            CommitTemplate {
                commit: data.messages.choose(&mut rand::thread_rng()).unwrap()
            };

            HttpResponse::Ok().content_type("text/html").body(s)

        }
        None => {
            let s = CommitTemplate {
                commit: data.messages.choose(&mut rand::thread_rng()).unwrap()
            }
                .render()
                .unwrap();

            CommitTemplate {
                commit: data.messages.choose(&mut rand::thread_rng()).unwrap()
            };

            HttpResponse::Ok().content_type("text/html").body(s)
        }
    }
}

#[get("/health")]
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
            .service(get_random_commit)
            .service(health)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
