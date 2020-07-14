#![feature(proc_macro_hygiene, decl_macro)]

mod shop;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::handlebars::{
    handlebars_helper, Helper, HelperResult, Output, RenderContext,
};
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        TemplateContext {
            title: "Cum Engineers - Home of the Cum Engine",
        },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render(
        "about",
        TemplateContext {
            title: "About the Cum Engineers",
        },
    )
}

#[catch(404)]
fn not_found() -> Template {
    Template::render(
        "404",
        TemplateContext {
            title: "Cum Engineers - Page Not Found",
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
