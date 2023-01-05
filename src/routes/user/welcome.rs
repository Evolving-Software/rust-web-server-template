// A route to send the user to after a successful login

use crate::routes::user::login::get_login;
use rocket::http::{Cookie, CookieJar, Status};
use rocket_dyn_templates::{context, Template};

#[get("/welcome")]
pub fn welcome(cookies: &CookieJar<'_>) -> Template {
    // if cookies has a username cookie, render the welcome page with the username
    // if not forward the user to the login page
    if let Some(cookie) = cookies.get("username") {
        Template::render(
            "welcome",
            context! {
                username: cookie.value(),
            },
        )
    } else {
        // Add an error cookie to the cookies with the value "You must login first"
        cookies.add(Cookie::new("error", "You must login first"));
        get_login(cookies)
    }
}
