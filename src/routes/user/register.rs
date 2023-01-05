// Register a user and return a JWT token
use rocket::form::Form;
use rocket_dyn_templates::Template;
use serde::Serialize;
use crate::models::user::User;