use crate::TemplateContext;
use anyhow::{Context as ErrorContext, Result};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

lazy_static! {
    static ref PRODUCTS: HashMap<String, Category> = load_products().unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Category {
    pretty_name: String,
    name: String,
    description: String,
    image: String,
    #[serde(default)]
    items: Vec<ShopItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ShopItem {
    name: String,
    price: f64,
    #[serde(default)]
    horsepower: u32,
    #[serde(default = "default_desc")]
    description: String,
    #[serde(default = "default_unit")]
    unit: String,
    image: Option<String>,
}

fn default_desc() -> String {
    "No description given.".to_owned()
}

fn default_unit() -> String {
    "liter".to_owned()
}

fn load_products() -> Result<HashMap<String, Category>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/products.yml");
    let file = File::open(path).context("Failed to open product data file")?;
    let reader = BufReader::new(file);
    let products =
        serde_yaml::from_reader(reader).context("Failed to parse yaml in product data file")?;
    Ok(products)
}

#[derive(Serialize)]
struct ShopOverviewContext {
    title: &'static str,
    categories: &'static HashMap<String, Category>,
}

#[derive(Serialize)]
struct ShopCategoryContext {
    title: String,
    categories: &'static HashMap<String, Category>,
    category: &'static Category,
    category_uri: String,
}

#[get("/shop")]
pub(crate) fn shop() -> Template {
    Template::render(
        "shop_overview",
        ShopOverviewContext {
            title: "The CumShop™",
            categories: &*PRODUCTS,
        },
    )
}

#[get("/shop/<category_uri>")]
pub(crate) fn shop_category(category_uri: String) -> Option<Template> {
    let category = PRODUCTS.get(&category_uri)?;
    Some(Template::render(
        "shop_category",
        ShopCategoryContext {
            title: format!("CumShop™ - {}", &category.pretty_name),
            categories: &*PRODUCTS,
            category,
            category_uri,
        },
    ))
}
