use actix_session::Session;

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use serde::Deserialize;

use super::db::r2d2_mongodb::mongodb::{
    bson, coll::options::FindOptions, db::ThreadedDatabase, doc, oid, Bson,
};

type Pool = super::db::r2d2::Pool<super::db::r2d2_mongodb::MongodbConnectionManager>;

pub fn index(req: HttpRequest, session: Session, db: web::Data<Pool>) -> HttpResponse {
    println!("{:?}", req);

    match session.get::<String>("token") {
        Ok(Some(id)) => {
            if cfg!(debug_assertions) {
                println!("ID Matched: {}", id);
            }

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
        _ => {
            if cfg!(debug_assertions) {
                println!("ID not matched");
            }

            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn register(
    req: HttpRequest,
    session: Session,
    data: web::Json<User>,
    db: web::Data<Pool>,
) -> HttpResponse {
    let conn = db.get().unwrap();
    println!("[REGISTER] {:?}", req);

    if cfg!(debug_assertions) {
        println!("{}", data.username);
    }

    let user_data = conn
        .collection("Users")
        .find_one(Some(doc! { "username" => data.username.clone() }), None)
        .unwrap();

    match user_data {
        Some(doc) => match doc.get("pass") {
            Some(pass) => {
                if cfg!(debug_assertions) {
                    println!("USER MATCHED");
                }

                // Chcek password
                let matches = argon2::verify_encoded(
                    pass.as_str().unwrap(),
                    data.password.clone().as_bytes(),
                )
                .unwrap();
                if matches {
                    if cfg!(debug_assertions) {
                        println!("Password matched");
                    }

                    match doc.get_object_id("_id").unwrap() {
                        oid => match session.set("token", oid.to_hex()) {
                            Ok(()) => HttpResponse::Ok().body("Welcome"),
                            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
                        },
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

pub fn logout(session: Session) -> HttpResponse {
    match session.set("token", "") {
        Ok(()) => HttpResponse::Found().header("location", "/").finish(),
        _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
}
