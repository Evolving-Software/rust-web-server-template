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
