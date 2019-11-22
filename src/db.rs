pub extern crate r2d2;
pub extern crate r2d2_mongodb;

use dotenv;

use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

pub fn init() -> r2d2::Pool<r2d2_mongodb::MongodbConnectionManager> {
    let db_host: &str = &dotenv::var("MONGO_HOST").expect("Env variable MONGO_HOST required");
    let db_port = dotenv::var("MONGO_PORT")
        .expect("Env variable MONGO_PORT required")
        .parse::<u16>()
        .unwrap();
    let db_name: &str = &dotenv::var("MONGO_DB").expect("Env variable MONGO_DB required");

    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(db_host, db_port)
            // .with_ssl(
            //     Some("path/to/ca.crt"),
            //     "path/to/client.crt",
            //     "path/to/client.key",
            //     VerifyPeer::Yes
            // )
            // .with_unauthenticated_ssl(
            //     Some("path/to/ca.crt"),
            //     VerifyPeer::No
            // )
            .with_db(db_name)
            .build(),
    );

    let pool = Pool::builder().max_size(16).build(manager).unwrap();
    pool
}
