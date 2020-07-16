use crate::Ads;
use rocket::State;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct HomeContext<'r> {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
    ads: &'r Vec<Ad>,
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
    Template::render(
        "index",
        HomeContext {
            title: "Cum Engineers",
            desc: "Cum Engineers - Home of the Cum Engine",
            image: "logo-dark.png",
            ads: ads.inner(),
        },
    )
}
