use crate::TemplateContext;
use rocket_contrib::templates::Template;

// TODO: improve this page
#[get("/about")]
pub fn index() -> Template {
    Template::render(
        "about",
        TemplateContext {
            title: "About the Cum Engineers",
            desc: "Cum Engineers - About the Cum Engineers",
            image: "sale.jpg",
        },
    )
}
