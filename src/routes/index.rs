use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
use rust_web::routes::login::login;


#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            foo: 123,
        },
    )
}