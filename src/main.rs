use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::header, web, App, HttpServer};
mod auth;
mod db;

use auth::{index, logout, register};
use dotenv;

extern crate argon2;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let port: &str = &dotenv::var("PORT").expect("Env variable PORT required");
    let domain = dotenv::var("DOMAIN").expect("Env variable DOMAIN required");

    let pool = db::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .domain(domain.clone())
                    .path("/")
                    .name("token")
                    .max_age(3600*9)
                    .secure(false),
            ))
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:4200")
                    .supports_credentials()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/login").route(web::post().to(register)))
            .service(web::resource("/logout").route(web::get().to(logout)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
}
