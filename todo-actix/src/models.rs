use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}
