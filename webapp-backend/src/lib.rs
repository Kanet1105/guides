mod login;
use login::get_oauth_client;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::{get, post, web};
use actix_web::dev::Server;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct UserInfo {
    state: String,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

#[get("/auth")]
async fn authorize_user(user_info: web::Query<UserInfo>) -> impl Responder {
    dbg!(user_info);
    HttpResponse::Ok()
}

#[get("/")]
async fn sign_in() -> impl Responder {
    HttpResponse::Found()
}

pub fn run_app() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        let oauth_client = web::Data::new(get_oauth_client().unwrap());
        App::new()
            .app_data(oauth_client)
            .service(authorize_user)
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}