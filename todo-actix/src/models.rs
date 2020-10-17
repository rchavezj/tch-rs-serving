use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize)]
pub struct Error {
    pub statuc: String,
}
