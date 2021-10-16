use crate::TemplateContext;
use rocket_contrib::templates::Template;

#[catch(404)]
pub fn not_found() -> Template {
    Template::render(
        "404",
        TemplateContext {
            title: "Page Not Found",
            desc: "Milk Engineers - Error 404, Page Not Found",
            image: "404.png",
        },
    )
}

#[catch(500)]
pub fn internal_error() -> Template {
    Template::render(
        "500",
        TemplateContext {
            title: "Internal Server Error",
            desc: "Milk Engineers - Error 500, Internal Server Error",
            image: "404.png",
        },
    )
}
