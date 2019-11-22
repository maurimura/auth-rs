use actix_identity::Identity;
use actix_web::{http::StatusCode, web, HttpResponse};
use serde::Deserialize;

use super::db::r2d2_mongodb::mongodb::{
    bson, coll::options::FindOptions, db::ThreadedDatabase, doc, oid, Bson,
};

type Pool = super::db::r2d2::Pool<super::db::r2d2_mongodb::MongodbConnectionManager>;

pub fn index(id: Identity, db: web::Data<Pool>) -> HttpResponse {
    println!("ENTRA ACA!");
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

            println!("MATCHEA!!");
            println!("{:?}", user_data);
            HttpResponse::Ok().json(user_data)
        }
        None => {
            println!("No ID matched");
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn register(id: Identity, data: web::Json<User>, db: web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();

    let user_data = conn
        .collection("Users")
        .find_one(Some(doc! { "username" => data.username.clone() }), None)
        .unwrap();

    match user_data {
        Some(doc) => match doc.get("pass") {
            Some(pass) => {
                // Chcek password
                let matches = argon2::verify_encoded(
                    pass.as_str().unwrap(),
                    data.password.clone().as_bytes(),
                )
                .unwrap();
                if matches {
                    match doc.get_object_id("_id").unwrap() {
                        oid => {
                            id.remember(oid.to_hex());
                            HttpResponse::Ok().body("Welcome")
                        }
                    }
                } else {
                    HttpResponse::new(StatusCode::UNAUTHORIZED)
                }
            }
            None => HttpResponse::new(StatusCode::UNAUTHORIZED),
        },
        None => HttpResponse::new(StatusCode::UNAUTHORIZED),
    }
}

pub fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}
#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
}
