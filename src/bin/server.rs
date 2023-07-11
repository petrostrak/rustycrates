use rocket_db_pools::Database;

extern crate rustwebapi;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rustwebapi::routes::rustaceans::get_rustaceans,
                rustwebapi::routes::rustaceans::get_rustacean_by_id,
                rustwebapi::routes::rustaceans::create_rustacean,
                rustwebapi::routes::rustaceans::update_rustacean,
                rustwebapi::routes::rustaceans::delete_rustacean,
                rustwebapi::routes::crates::get_crates,
                rustwebapi::routes::crates::get_crate_by_id,
                rustwebapi::routes::crates::create_crate,
                rustwebapi::routes::crates::update_crate,
                rustwebapi::routes::crates::delete_crate,
                rustwebapi::routes::authorization::login
            ],
        )
        .attach(rustwebapi::routes::DB::fairing())
        .attach(rustwebapi::routes::Cache::init())
        .launch()
        .await;
}
