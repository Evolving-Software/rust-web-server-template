use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/contact")]
pub fn get_contact_page() -> Template {
    Template::render(
        "contact",
        context! {
            foo: 123,
        },
    )
}
