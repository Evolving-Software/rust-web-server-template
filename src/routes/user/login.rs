use rocket_dyn_templates::{context, Template};
use rocket::form::Form;
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





#[derive(FromForm)]
struct UserLogin {
    username: String,
    password: String,
}

// User Login POST
#[post("/login", data = "<login>")]
fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if login.username == "Sergio" && login.password == "password" {
        jar.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(Redirect::to(uri!(login)), "Invalid username/password."))
    }
}