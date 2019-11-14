use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn index(id: Identity) -> HttpResponse {
    // format!(
    //     "Hello {}",
    //     id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    // );
    match id.identity() {
        Some(id) => HttpResponse::Ok().body(id),
        None => HttpResponse::new(StatusCode::UNAUTHORIZED),
    }
}

fn register(id: Identity, data: web::Path<User>) -> HttpResponse {
    println!("Llega aca");
    id.remember(data.id.to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}
#[derive(Deserialize)]
struct User {
    id: String,
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("token")
                    .secure(false),
            ))
            // enable logger - always register actix-web Logger middleware last
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{id}").route(web::post().to(register)))
            .service(web::resource("/logout").route(web::get().to(logout)))
    })
    .bind("127.0.0.1:3000")?
    .run()
}
