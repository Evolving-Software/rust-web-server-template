// Copyright (C) 2022 RDS Ventures LLC
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use actix_files as fs;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    error,
    http::{header::ContentType, StatusCode},
    middleware::{self, ErrorHandlerResponse, ErrorHandlers},
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use actix_web_lab::respond::Html;
use std::collections::HashMap;
use tera::Tera;

// store tera template in application state
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

async fn greet(req: HttpRequest) -> impl Responder {
    format!("Hello {}!", req.match_info().get("name").unwrap())
}

/// Health check endpoint
/// Returns 200 OK if the service is running
/// Returns 500 Internal Server Error if the service is not running
/// This is used by the load balancer to determine if the service is healthy
/// and should be routed to or not.
/// This is also used by the Kubernetes liveness probe to determine if the
/// service is healthy and should be restarted or not.
///
/// # Arguments
///
/// * `req` - The HTTP request
///
/// # Returns
///
/// * `impl Responder` - The HTTP response
///
/// # Example
///
/// ```
/// use actix_web::{web, App, HttpRequest, HttpServer, Responder};
/// async fn health_check(req: HttpRequest) -> impl Responder {
///    format!("OK")
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the service is not running.
///
/// # Errors
///
/// This function will return an error if the service is not running.
async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();

        App::new()
            .service(fs::Files::new("/public", "./public").show_files_listing())
            .app_data(web::Data::new(tera))
            .route("/health_check", web::get().to(health_check))
            .wrap(middleware::Logger::default()) // enable logger
            .route("/", web::get().to(init_tera))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
