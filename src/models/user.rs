use rocket::response::{Redirect, Flash};
use rocket_db_pools::{sqlx, Database, Connection};



// let conn = SqliteConnection::connect("sqlite::memory:").await?;

pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    created_at: String,
    updated_at: String,
}
#[derive(FromForm)]
pub struct UserLogin {
    username: String,
    password: String,
}

impl UserLogin {
    pub fn login(&self) -> Result<Redirect, Flash<Redirect>> {
        if self.username == "Riley" && self.password == "123" {
            Ok(Redirect::to("/"))
        } else {
            //redirect to index with error message
            Err(Flash::error(Redirect::to("/"), "Invalid username or password"))
        }
    }
}