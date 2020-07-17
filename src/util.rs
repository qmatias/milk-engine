use crate::{Ads, DbConn, Products};
use anyhow::{Context as ErrorContext, Result};
use chrono::Duration;
use log::error;
use rocket::fairing::Fairing;
use rocket::{Cargo, Rocket};
use rocket_contrib::templates::handlebars::handlebars_helper;
use rocket_contrib::templates::Template;
use serde::de::DeserializeOwned;
use std::path::Path;
use tokio::fs;

// Macro from `diesel_migrations` which allows the server to be run and
// tested without any outside setup of the database.
embed_migrations!();

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
pub async fn attach_db_migrations(mut rocket: Rocket) -> Result<Rocket, Rocket> {
    match run_db_migrations(rocket.inspect().await) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Error while running database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn run_db_migrations(cargo: &Cargo) -> Result<()> {
    let conn = DbConn::get_one(cargo).context("Failed to get a connection from pool")?;
    embedded_migrations::run(&*conn).context("Failed to run database migrations")?;
    Ok(())
}

/// adhoc fairing to load static data
pub async fn attach_static_data(rocket: Rocket) -> Result<Rocket, Rocket> {
    match read_static_data().await {
        Ok((products, ads)) => Ok(rocket.manage(products).manage(ads)),
        Err(e) => {
            error!("Failed to load static data: {:?}", e);
            Err(rocket)
        }
    }
}

async fn read_static_data() -> Result<(Products, Ads)> {
    tokio::try_join!(
        read_yaml::<Products>("data/products.yml"),
        read_yaml::<Ads>("data/ads.yml"),
    )
}

/// Loads arbitrary serialized data from a path relative to crate root
async fn read_yaml<T: DeserializeOwned>(path: &'static str) -> Result<T> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let contents = fs::read(path)
        .await
        .context("Failed to read product data file")?;
    serde_yaml::from_slice(&contents).context("Failed to parse yaml in product data file")
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
