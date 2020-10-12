use serde::Deserialize;

#[derive(Deserialize)]
pub struct Status {
    pub status: String,
}
