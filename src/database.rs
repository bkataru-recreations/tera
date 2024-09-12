use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::sql::{thing, Datetime, Thing, Uuid};
use surrealdb::Surreal;

lazy_static::lazy_static! {
    pub static ref DB: async_once::AsyncOnce<Surreal<Db>> = async_once::AsyncOnce::new(async {
        let db = connect_db().await.expect("Unable to connect to database");
        db
    });
}