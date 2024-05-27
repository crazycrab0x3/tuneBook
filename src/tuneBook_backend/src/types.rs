use candid::{CandidType, Deserialize};

#[derive(CandidType, Clone, Debug)]
pub struct Profile {
    pub principal: String,
    pub username: String,
    pub avatar: Vec<u8>
}
#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct Tune {
    pub origin: bool,
    pub title: String,
    pub tune_data: Option<String>,
    pub timestamp: String
}