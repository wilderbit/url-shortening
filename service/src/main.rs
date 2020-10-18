use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info,actix_web=debug,actix_server=debug")
        .write_style_or("RUST_LOG_STYLE", "always");
    let mut builder = env_logger::Builder::from_env(env);
    builder.target(env_logger::Target::Stdout);
    builder.init();

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
