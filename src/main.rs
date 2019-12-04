use actix_cors::Cors;
use actix_http::cookie::SameSite;
use actix_session::{CookieSession, Session};
use actix_web::{http::header, web, App, HttpServer};
use auth::{index, logout, register};
use dotenv;

mod auth;
mod db;

extern crate argon2;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let port: &str = &dotenv::var("PORT").expect("Env variable PORT required");
    // let domain = dotenv::var("DOMAIN").expect("Env variable DOMAIN required");

    let pool = db::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(CookieSession::signed(&[0; 32])
                    .path("/")
                    .name("token")
                    .max_age(3600 * 9)
                    .same_site(SameSite::Lax)
                    .secure(cfg!(not(debug_assertions))),
            )
            .wrap(
                Cors::new()
                    .allowed_origin("http://v2.fundar.com.ar")
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
