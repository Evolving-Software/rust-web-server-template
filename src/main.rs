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

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
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
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
