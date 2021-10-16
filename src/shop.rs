use crate::Products;
use rocket::State;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

/// A category in the shop
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    /// Pretty name of the category to be displayed
    pub pretty_name: String,

    /// Name of the category which shouldn't change
    pub name: String,

    /// Brief category description
    pub description: String,

    /// Category preview image
    pub image: Option<String>,

    /// All of the products which belong to this category
    #[serde(default)]
    pub items: Vec<Product>,
}

/// A product in the shop
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    /// Pretty name of the product
    pub name: String,

    /// Price of product
    pub price: f64,

    /// Horsepower, displayed for engine listings
    #[serde(default)]
    pub horsepower: u32,

    /// Description shown on the product preview page
    #[serde(default = "default_desc")]
    pub description: String,

    /// Unit, displayed for milk listings
    #[serde(default = "default_unit")]
    pub unit: String,

    /// An image for the product
    pub image: Option<String>,
}

fn default_desc() -> String {
    "No description given.".to_owned()
}

fn default_unit() -> String {
    "Liter".to_owned()
}

#[derive(Serialize)]
struct ShopOverviewContext<'r> {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
    categories: &'r HashMap<String, Category>,
}

#[derive(Serialize)]
struct ShopCategoryContext<'r> {
    title: String,
    desc: &'r str,
    image: Option<&'r str>,
    categories: &'r HashMap<String, Category>,
    category: &'r Category,
    category_uri: String,
}

#[get("/shop")]
pub fn index(products: State<Products>) -> Template {
    Template::render(
        "shop_overview",
        ShopOverviewContext {
            title: "The MilkShop™",
            image: "sale.jpg",
            desc: "Our MilkShop™, where you can shop for all of our products",
            categories: &products,
        },
    )
}

#[get("/shop/<category_uri>")]
pub fn category(category_uri: String, products: State<Products>) -> Option<Template> {
    let category = products.get(&category_uri)?;
    Some(Template::render(
        "shop_category",
        ShopCategoryContext {
            title: format!("MilkShop™ - {}", &category.pretty_name),
            image: category.image.as_deref(),
            desc: &category.description,
            categories: &products,
            category,
            category_uri,
        },
    ))
}
