#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;
use crate::models::Asset;
use crate::schema::assets::dsl::assets;
pub use db::models;
pub use db::schema;
//use db::*;

pub fn connect_sqlite() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_assets(connection: &SqliteConnection) -> Vec<Asset> {
    let results = assets
        //.filter(published.eq(true))
        .limit(100)
        .load::<Asset>(connection)
        .expect("Error loading assets");
    println!("There were(was) {} asset(s) loaded", results.len());
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
