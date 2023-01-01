use crate::routes::health_check;
use actix_files as fs;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{error, middleware, web, App, Error, HttpServer, Responder};
use actix_web_lab::respond::Html;
use std::collections::HashMap;
use std::net::TcpListener;
use tera::Tera;

async fn init_tera(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, Error> {
    let s: String = if let Some(name) = query.get("name") {
        // Create context to store the state.
        let mut ctx = tera::Context::new();
        // Insert name into the context
        ctx.insert("name", name);
        // Insert text into the context.
        ctx.insert("text", "Welcome!");
        // Render the template and the context.
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template Error"))?
    } else {
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };

    Ok(Html(s))
}

#[tokio::main]
pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    let server = HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();
        App::new()
            .service(fs::Files::new("/public", "./public").show_files_listing())
            .app_data(web::Data::new(tera))
            .route("/health_check", web::get().to(health_check))
            .wrap(middleware::Logger::default()) // enable logger
            .route("/", web::get().to(init_tera))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
