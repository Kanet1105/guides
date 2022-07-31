mod login;
use login::{get_oauth_client, get_login_url};
use login::UserInfo;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
use env_logger;
use oauth2::basic::BasicClient;

/// "/"
async fn index(oauth_client: web::Data<BasicClient>) -> impl Responder {
    let client = oauth_client.get_ref();
    let redirect_url = get_login_url(client);
    println!("===========================================================================");
    println!("{}", redirect_url.to_string());
    println!("===========================================================================");
    HttpResponse::Ok().finish()
}

/// "/auth"
async fn auth(user_info: web::Query<UserInfo>) -> impl Responder {
    println!("===========================================================================");
    println!("{:?}", &user_info);
    println!("===========================================================================");
    HttpResponse::Ok().finish()
}

pub fn run_app() -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    println!("===========================================================================");
    println!("Starting the http server at 'http://127.0.0.1:8080'.");
    println!("===========================================================================");

    let server = HttpServer::new(|| {
        let oauth_client = web::Data::new(get_oauth_client().unwrap());
        App::new()
            .wrap(Logger::default())
            .app_data(oauth_client)
            .route("/", web::get().to(index))
            .route("/auth", web::get().to(auth))
    })
    .bind("127.0.0.1:8080")?
    .run();
    Ok(server)
}