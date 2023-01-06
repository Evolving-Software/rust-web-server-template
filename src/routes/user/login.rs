use crate::models::user::UserLogin;
use rocket::{
    form::Form,
    http::CookieJar,
    response::{Flash, Redirect},
};
use rocket_dyn_templates::{context, Template};

// Get Login page, if the cookie is set, redirect to welcome page
// If not, render the login page
// if the cookie has an error, render the login page with the error
#[get("/login")]
pub fn get_login(cookies: &CookieJar<'_>) -> Template {
    if let Some(cookie) = cookies.get("username") {
        Template::render(
            "welcome",
            context! {
                username: cookie.value(),
            },
        )
    } else if let Some(cookie) = cookies.get("error") {
        Template::render(
            "login",
            context! {
                error: cookie.value(),
            },
        )
    } else {
        Template::render(
            "login",
            context! {
                error: "",
            },
        )
    }
}

// Post Login page
#[post("/login", data = "<user_login_data>")]
pub fn post_login(
    _jar: &CookieJar<'_>,
    user_login_data: Form<UserLogin>,
) -> Result<Template, Flash<Redirect>> {
    user_login_data.user_login()
}
