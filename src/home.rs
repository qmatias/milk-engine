use crate::Ads;
use rocket::State;
use rocket_contrib::templates::Template;
use rand::prelude::*;

#[derive(Serialize)]
struct HomeContext<'r> {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
    ads: &'r Vec<Ad>,
	sale_image: &'static str,
	sale_link: &'static str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ad {
    pub pretty_name: String,
    pub desc: String,
    pub image: String,
    pub link: String,
}

#[get("/")]
pub fn index(ads: State<Ads>) -> Template {
	let sales = vec![
		("sale-milk.jpg", "/shop/milk"),
		("sale-engine.jpg", "/shop/engines")
	];
	let sale = sales.choose(&mut thread_rng()).unwrap();
    Template::render(
        "index",
        HomeContext {
            title: "Milk Engineers",
            desc: "Milk Engineers - Home of the Milk Engine",
            image: "icon.png",
            ads: ads.inner(),
			sale_image: sale.0,
			sale_link: sale.1,
        },
    )
}
