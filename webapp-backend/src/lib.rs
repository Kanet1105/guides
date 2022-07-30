mod login;
use login::get_oauth_client;

use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web::web;
use serde::Deserialize;
use oauth2::basic::BasicClient;

#[derive(Debug, Deserialize)]
struct UserInfo {
    state: String,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

// "/"
async fn index(oauth_client: web::Data<BasicClient>) -> std::io::Result<NamedFile> {
    println!("{:?}", &oauth_client);
    let index_path = std::path::PathBuf::from("/static/index.html");
    Ok(NamedFile::open(index_path)?)
}

// "/auth"
async fn authorize(user_info: web::Data<UserInfo>) -> impl Responder {
    println!("{:?}", &user_info);
    HttpResponse::Ok().finish()
}

pub fn run_app() -> Result<Server, std::io::Error> {
    println!("Starting the http server at 'http://127.0.0.1:8080'.");

    let server = HttpServer::new(|| {
        let oauth_client = web::Data::new(get_oauth_client().unwrap());
        App::new()
            .app_data(oauth_client)
            .route("/", web::get().to(index))
            .route("/auth", web::get().to(authorize))
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}