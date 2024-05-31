use candid::{ Principal, CandidType, Deserialize};

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

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct OnSetDocContext {
    pub caller: Principal,
    pub data: DocContext
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct DocContext {
    pub collection: String,
    pub key: String,
    pub data: DocUpsert
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct DocUpsert {
    pub before: Option<Doc>,
    pub after: Doc
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct Doc {
    pub owner: Principal,
    pub data: Vec<u8>,
    pub description: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub version: Option<u64>,
}