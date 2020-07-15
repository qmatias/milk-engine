#![feature(proc_macro_hygiene, decl_macro)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::non_ascii_literal, clippy::cargo_common_metadata)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

mod shop;

mod home;

#[cfg(test)]
mod tests;

use anyhow::{Context as ErrorContext, Result};
use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::handlebars::handlebars_helper;
use rocket_contrib::templates::Template;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

lazy_static! {
    pub static ref PRODUCTS: HashMap<String, shop::Category> =
        load_data("data/products.yml").unwrap();
    pub static ref ADS: Vec<home::Ad> = load_data("data/ads.yml").unwrap();
}

fn load_data<T: DeserializeOwned>(path: &'static str) -> Result<T> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let file = File::open(path).context("Failed to open product data file")?;
    let reader = BufReader::new(file);
    let products =
        serde_yaml::from_reader(reader).context("Failed to parse yaml in product data file")?;
    Ok(products)
}

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
}

#[get("/about")]
fn about() -> Template {
    Template::render(
        "about",
        TemplateContext {
            title: "About the Cum Engineers",
            desc: "Cum Engineers - About the Cum Engineers",
            image: "sale.jpg",
        },
    )
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "404",
        TemplateContext {
            title: "Page Not Found",
            desc: "Cum Engineers - Error 404, Page Not Found",
            image: "404.png",
        },
    )
}

#[catch(500)]
fn internal_error() -> Template {
    Template::render(
        "500",
        TemplateContext {
            title: "Internal Server Error",
            desc: "Cum Engineers - Error 500, Internal Server Error",
            image: "404.png",
        },
    )
}

fn main() {
    rocket().launch();
}

fn rocket() -> Rocket {
    lazy_static::initialize(&PRODUCTS);
    lazy_static::initialize(&ADS);

    handlebars_helper!(str_eq: |x: str, y: str| x == y);

    let template_engine = Template::custom(|engines| {
        engines.handlebars.set_strict_mode(true);
        engines
            .handlebars
            .register_helper("str_eq", Box::new(str_eq));
    });

    rocket::ignite()
        .mount(
            "/",
            routes![home::index, about, shop::shop, shop::shop_category],
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .register(catchers![not_found, internal_error])
        .attach(template_engine)
}
