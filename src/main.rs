use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};

use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    data: String,
}


fn index(id: Identity) -> String {
    format!(
        "Hello {}",
        id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    )
}

fn from() -> HttpResponse {
    let form = String::from(
        "<form action=\"/login\" method=\"post\">
    <input type=\"text\" name=\"data\" value=\"mauri\" />
    <input type=\"submit\" />
  </form>",
    );

    HttpResponse::Ok().body(form)
}

fn login(id: Identity, form: web::Form<FormData>) -> HttpResponse {
    println!("{}", form.data);

    id.remember(form.data.to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-example")
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
