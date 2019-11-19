use actix_identity::Identity;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};
use serde::Deserialize;
mod db;
use db::r2d2_mongodb::mongodb::{
    bson, coll::options::FindOptions, db::ThreadedDatabase, doc, oid, Bson };

type Pool = db::r2d2::Pool<db::r2d2_mongodb::MongodbConnectionManager>;

fn index(id: Identity, db: web::Data<Pool>) -> HttpResponse {
    match id.identity() {
        Some(id) => {
            let conn = db.get().unwrap();
            let filter =
                Some(doc! { "_id" => Bson::ObjectId(oid::ObjectId::with_string(&id).unwrap()) });
            let mut projection = FindOptions::new();
            projection.projection = Some(doc! {
                "pass" => false,
            });

            let user_data = conn
                .collection("Users")
                .find_one(filter, Some(projection))
                .unwrap();

            HttpResponse::Ok().json(user_data)
        }
        None => {
            println!("No ID matched");
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

fn register(id: Identity, data: web::Json<User>, db: web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();

    let user_data = conn
        .collection("Users")
        .find_one(Some(doc! { "username" => data.username.clone() }), None)
        .unwrap();

    match user_data {
        Some(doc) => match doc.get_object_id("_id").unwrap() {
            oid => {
                id.remember(oid.to_hex());
                HttpResponse::Ok().body("Welcome")
            }
        },
        None => HttpResponse::new(StatusCode::UNAUTHORIZED),
    }

    // id.remember(data.id.to_owned());
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
