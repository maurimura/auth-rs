use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

fn login(id: Identity, data: web::Form<FormData>) -> HttpResponse {
    id.remember(data.name.to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}
#[derive(Deserialize)]
struct FormData {
    name: String,
}

fn from() -> HttpResponse {
    let form = String::from(
        "<form action=\"/login\" method=\"post\">
    <input type=\"text\" name=\"name\" value=\"\" />
    <input type=\"submit\" />
  </form>",
    );

    HttpResponse::Ok().body(form)
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
            .wrap(middleware::Logger::default())
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").to(logout))
            .service(web::resource("/signup").route(web::get().to(from)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:3000")?
    .run()
}
