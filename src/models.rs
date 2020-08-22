use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Status{
    pub status: String,
}
