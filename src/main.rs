use routes::{
    crates::{create_crate, delete_crate, get_crate_by_id, get_crates, update_crate},
    rustaceans::*,
    DB,
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                get_rustacean_by_id,
                create_rustacean,
                update_rustacean,
                delete_rustacean,
                get_crates,
                get_crate_by_id,
                create_crate,
                update_crate,
                delete_crate
            ],
        )
        .attach(DB::fairing())
        .launch()
        .await;
}
