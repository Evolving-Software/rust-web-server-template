use rocket::http::Status;

pub fn health_check() -> (Status, &'static str) {
    (Status::Ok, "OK")
}
