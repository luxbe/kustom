use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub version: u8,
    pub title: String,
    pub description: String,
    pub author: String,
    pub email: String,
    pub width: u32,
    pub height: u32,
    // a string of space seperated values for the different features used
    pub features: String,
    pub release: u32,
    pub locked: bool,
    pub pflags: u8,
}