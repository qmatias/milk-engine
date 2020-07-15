#![feature(proc_macro_hygiene, decl_macro)]

mod shop;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::handlebars::handlebars_helper;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        TemplateContext {
            title: "Cum Engineers",
            desc: "Cum Engineers - Home of the Cum Engine",
            image: "sale.jpg",
        },
    )
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
    missing_page()
}

#[catch(500)]
fn server_error() -> Template {
    missing_page()
}

fn missing_page() -> Template {
    Template::render(
        "404",
        TemplateContext {
            title: "Page Not Found",
            desc: "Cum Engineers - Error 404, Page Not Found",
            image: "404.png",
        },
    )
}

fn main() {
    handlebars_helper!(str_eq: |x: str, y: str| x == y);

    let template_engine = Template::custom(|engines| {
        engines.handlebars.set_strict_mode(true);
        engines
            .handlebars
            .register_helper("str_eq", Box::new(str_eq));
    });

    rocket::ignite()
        .mount("/", routes![index, about, shop::shop, shop::shop_category])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .register(catchers![not_found])
        .attach(template_engine)
        .launch();
}

fn empty_ctx() -> HashMap<(), ()> {
    HashMap::new()
}
