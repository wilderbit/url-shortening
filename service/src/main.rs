use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

mod base;
mod routes;

#[derive(Deserialize)]
pub struct CreateUrl {
    pub api_key: Option<String>,
    pub url: String,
    pub alias: Option<String>,
}

#[get("/{id}/{name}/")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

async fn create(web::Query(info): web::Query<CreateUrl>) -> impl Responder {
    format!(
        "api_key {}! url:{}, alias: {}",
        info.api_key.unwrap_or("none".to_string()),
        info.url,
        info.alias.unwrap_or("".to_string())
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info,actix_web=debug,actix_server=debug")
        .write_style_or("RUST_LOG_STYLE", "always");
    let mut builder = env_logger::Builder::from_env(env);
    builder.target(env_logger::Target::Stdout);
    builder.init();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(web::resource("/abc").route(web::get().to(create)))
            .service(web::resource("/create").route(web::post().to(routes::create::url)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
