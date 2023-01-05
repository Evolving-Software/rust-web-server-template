use crate::routes::login::login;
use crate::routes::{health_check, user };
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        foo: 123,
    })
}

