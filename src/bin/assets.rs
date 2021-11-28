extern crate cristo;
extern crate diesel;

//use self::diesel::prelude::*;
//use cristo::models::*;
use cristo::*;

fn main() {
    //use cristo::schema::assets::dsl::*;

    let connection = connect_sqlite();
    let ass = get_assets(&connection);
    if !ass.is_empty() {
        println!("Displaying {} assets", ass.len());
        for asset in ass {
            println!("[{:?}] {} ({})", asset.id, asset.ticker, asset.fullname);
        }
    }
}
