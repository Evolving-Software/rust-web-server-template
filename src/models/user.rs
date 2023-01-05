use sqlx::Connection;
use rocket_db_pools::{sqlx, Database, Connection};
let conn = SqliteConnection::connect("sqlite::memory:").await?;

struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    created_at: String,
    updated_at: String,
}

struct UserLogin {
    username: String,
    password: String,
}

// 