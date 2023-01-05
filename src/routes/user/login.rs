use crate::models::user::UserLogin;
use rocket::{form::Form, response::{Redirect, Flash}};
use rocket_dyn_templates::{context, Template};

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
pub fn login_post(user_login: Form<UserLogin>) -> Result<Redirect, Flash<Redirect>> {
    UserLogin::user_Login(&user_login)
}
