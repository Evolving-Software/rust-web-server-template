use rocket::{form::Form};
use rocket_dyn_templates::{context, Template};
use crate::models::user::UserLogin;

// Get Login page
#[get("/login")]
pub fn login() -> Template {
    Template::render(
        "login",
        context! {
            foo: 123,
        },
    )
}


// Post Login page
#[post("/login", data = "<user_login>")]
pub fn login_post(user_login: Form<UserLogin>) -> Template {
    Template::render(
        "login",
        context! {
            foo: 123,
        },
    )
}