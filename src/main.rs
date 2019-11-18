use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
mod db;
use db::r2d2_mongodb::mongodb::{db::ThreadedDatabase, doc, bson, Bson};
use serde_json::json;

type Pool = db::r2d2::Pool<db::r2d2_mongodb::MongodbConnectionManager>;

fn index(
    id: Identity,
    db: web::Data<Pool>,
) -> HttpResponse {
    // format!(
    //     "Hello {}",
    //     id.identity().unwrap_or_else(|| "Anonymous".to_owned())
    // );
    match id.identity() {
        Some(id) => {
            let conn = db.get().unwrap();
            let user_data = conn.collection("Users").find_one(None, None).unwrap();
            HttpResponse::Ok().json(user_data)
        }
        None => HttpResponse::new(StatusCode::UNAUTHORIZED),
    }
}

fn register(id: Identity, data: web::Json<User>, db: web::Data<Pool>) -> HttpResponse {
    println!("Llega aca");
    println!("{} {}", data.username, data.password);
    let conn = db.get().unwrap();
    
    let user_data = conn.collection("Users").find_one(Some(doc!{ "username" => data.username.clone() }), None).unwrap();
    println!("{:?}", user_data);

    match user_data {
        Some(doc) => match doc.get("_id") {
            Some(mongo_id) => println!("{}", mongo_id),
            _ => println!("_id not found") 
        },
        None => println!("User not match")
    }


    // id.remember(data.id.to_owned());
    HttpResponse::Found().header("location", "/").finish()
}

fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}
#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    let pool = db::init();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("token")
                    .secure(false),
            ))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/login").route(web::post().to(register)))
            .service(web::resource("/logout").route(web::get().to(logout)))
    })
    .bind("127.0.0.1:3000")?
    .run()
}
