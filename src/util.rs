use crate::{Ads, DbConn, Products};
use anyhow::{Context as ErrorContext, Result};
use chrono::Duration;
use log::error;
use rocket::fairing::Fairing;
use rocket::Rocket;
use rocket_contrib::templates::handlebars::handlebars_helper;
use rocket_contrib::templates::Template;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;

// Macro from `diesel_migrations` which allows the server to be run and
// tested without any outside setup of the database.
embed_migrations!();

fn _load_static_data() -> Result<(Products, Ads)> {
    Ok((
        load_yaml_from::<Products>("data/products.yml")?,
        load_yaml_from::<Ads>("data/ads.yml")?,
    ))
}

pub fn build_template_engine() -> impl Fairing {
    handlebars_helper!(str_eq: |x: str, y: str| x == y);

    Template::custom(|engines| {
        engines.handlebars.set_strict_mode(true);
        engines
            .handlebars
            .register_helper("str_eq", Box::new(str_eq));
    })
}

/// adhoc fairing to run diesel migrations
pub fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("failed to connect to database");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

/// Loads arbitrary serialized data from a path relative to crate root
fn load_yaml_from<T: DeserializeOwned>(path: &'static str) -> Result<T> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let file = File::open(path).context("Failed to open product data file")?;
    let reader = BufReader::new(file);
    let products =
        serde_yaml::from_reader(reader).context("Failed to parse yaml in product data file")?;
    Ok(products)
}

/// adhoc fairing to load static data
pub fn load_static_data(rocket: Rocket) -> Result<Rocket, Rocket> {
    match _load_static_data() {
        Ok((products, ads)) => Ok(rocket.manage(products).manage(ads)),
        Err(e) => {
            error!("Failed to load static data: {:?}", e);
            Err(rocket)
        }
    }
}

pub fn to_bulma_class(flash_name: &str) -> &'static str {
    match flash_name {
        "error" => "danger",
        "warning" => "warning",
        "success" => "success",
        "link" => "link",
        "info" => "info",
        _ => "primary",
    }
}

pub fn convert_ip(address: SocketAddr) -> Vec<u8> {
    match address.ip() {
        IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
        IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
    }
}

pub fn format_duration(d: Duration) -> String {
    if d.num_seconds() <= 5 {
        "Just now".to_owned()
    } else if d.num_minutes() <= 0 {
        format!("{}s", d.num_seconds())
    } else if d.num_hours() <= 0 {
        format!("{}m", d.num_minutes())
    } else if d.num_days() <= 1 {
        format!("{}h", d.num_hours())
    } else if d.num_weeks() <= 2 {
        format!("{}d", d.num_days())
    } else {
        format!("{}w", d.num_weeks())
    }
}
