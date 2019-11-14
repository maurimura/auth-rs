extern crate r2d2;
extern crate r2d2_mongodb;



use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager, VerifyPeer, mongodb::db::ThreadedDatabase};

pub fn init() -> r2d2::Pool<r2d2_mongodb::MongodbConnectionManager> {
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host("localhost", 27017)
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
            .with_db("FUNDAR")
            .build(),
    );

    let pool = Pool::builder().max_size(16).build(manager).unwrap();
    pool
}
