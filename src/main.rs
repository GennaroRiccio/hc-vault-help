mod secret;

use std::io::Error;
use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use crate::secret::helper::{Config, get_config, get_vault_connection, Secrets};
static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize,Serialize)]
struct Path{
    engine : String,
    secret:  String,
    key :  String,
    value:  String,
}

async fn index() -> Result<HttpResponse,Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}
async fn add(path: web::Form<Path>) -> Result<HttpResponse, Error> {
    println!("{:?}",path.key);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Secret Definition {:?}", path.key)))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/add", web::post().to(add))
    })
        .bind(("localhost", 8090))
        .unwrap()
        .workers(2);
    println!("Server live at http://{}", "localhost:8090".to_string());
    let _ = server.run().await;

    Ok(())
}


