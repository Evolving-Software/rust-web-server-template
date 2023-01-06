// Register a user and return a JWT token
use crate::models::user::User;
use rocket::form::Form;
use rocket_dyn_templates::Template;
use serde::Serialize;
