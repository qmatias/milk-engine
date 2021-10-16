use crate::TemplateContext;
use rocket_contrib::templates::Template;

// TODO: improve this page
#[get("/about")]
pub fn index() -> Template {
    Template::render(
        "about",
        TemplateContext {
            title: "About the Milk Engineers",
            desc: "Milk Engineers - About the Milk Engineers",
            image: "sale.jpg",
        },
    )
}
