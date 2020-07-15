use crate::ADS;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct HomeContext {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
    ads: &'static Vec<Ad>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ad {
    pub pretty_name: String,
    pub desc: String,
    pub image: String,
    pub link: String,
}

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        HomeContext {
            title: "Cum Engineers",
            desc: "Cum Engineers - Home of the Cum Engine",
            image: "sale.jpg",
            ads: &*ADS,
        },
    )
}
