#![feature(proc_macro_hygiene, decl_macro, exclusive_range_pattern)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::non_ascii_literal)] // it's 2020
#![allow(clippy::cargo_common_metadata)] // not a crate
#![allow(clippy::needless_pass_by_value)] // fucks with rocket

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod about;
mod comments;
mod errors;
mod home;
mod schema;
mod shop;
mod util;

#[cfg(test)]
mod tests;

use diesel::SqliteConnection;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

/// Map of URL to shop categories, lazy loaded from data/products.yml
type Products = HashMap<String, shop::Category>;

/// List of ads to show on the home screen, lazy loaded from data/ads.yml
type Ads = Vec<home::Ad>;

/// Very basic template context
#[derive(Serialize)]
pub struct TemplateContext {
    pub title: &'static str,
    pub desc: &'static str,
    pub image: &'static str,
}

fn main() {
    build_rocket().launch();
}

fn build_rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach(
            "Perform Database Migrations",
            util::run_db_migrations,
        ))
        .attach(AdHoc::on_attach("Load Static Data", util::load_static_data))
        .mount(
            "/",
            routes![
                home::index,
                about::index,
                comments::index,
                comments::post,
                shop::index,
                shop::category
            ],
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .register(catchers![errors::not_found, errors::internal_error])
        .attach(util::build_template_engine())
}
