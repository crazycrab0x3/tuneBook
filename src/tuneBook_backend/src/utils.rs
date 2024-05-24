use std::cell::RefCell;
use std::collections::BTreeMap;
use ic_cdk;
use std::env;
use std::fs;
use std::path::Path;
use serde_json::{Value, json};
use crate::types;


type ProfileStore = BTreeMap<String, types::Profile>;
type TuneDB = BTreeMap<String, String>;

thread_local! {
    pub static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    pub static TUNE_DB: RefCell<TuneDB> = RefCell::default();
}

const Tune_DB: &str = include_str!("./tune_db.json");

pub async fn init(){

    ic_cdk::setup();
    let parsed: Value = serde_json::from_str(Tune_DB).expect("parse error!");
    TUNE_DB.with(|tune_db|{


        let btree_map: BTreeMap<String, String> = if let Value::Object(obj) = parsed {
            obj.into_iter()
                .map(|(k, v)| (k, v.as_str().unwrap().to_string()))
                .collect()
        } else {
            eprintln!("Expected a JSON object");
            BTreeMap::new() // Return an empty map if not an object
        };
        *tune_db.borrow_mut() = btree_map;

    });
}

pub fn authentication(principal: String) -> Option<types::Profile> {
    PROFILE_STORE.with(|profile_store| {
        if profile_store.borrow().get(&principal).is_some(){
            Some(profile_store.borrow().get(&principal).unwrap().clone())
        }
        else{
            None
        }
    })
}

pub async fn update_profile(principal:String, username: String, avatar: Vec<u8>) -> types::Profile {
    PROFILE_STORE.with(|profile_store| {
        if profile_store.borrow().get(&principal).is_some(){
            let user_profile = profile_store.borrow().get(&principal).unwrap().clone();
            
            let new_profile = types::Profile{
                principal: principal.clone(),
                username,
                avatar,
                tune_book: user_profile.tune_book
            };
            profile_store.borrow_mut().insert(principal, new_profile.clone());
            new_profile
        }
        else{
            let new_profile = types::Profile{
                principal: principal.clone(),
                username,
                avatar,
                tune_book: vec![]
            };
            profile_store.borrow_mut().insert(principal, new_profile.clone());
            new_profile            
        }
    })
}

pub fn get_original_tune_list(page_number: usize) -> Vec<String> {
    TUNE_DB.with(|tune_db| {
        let res: Vec<String> = tune_db.borrow()
        .iter()
        .skip(page_number*50)
        .enumerate()
        .filter_map(|(index, (title, _))| {
            if index < 50 {
                Some(title.clone())
            }
            else{
                None
            }
        })
        .collect();
        res
    })
}

pub fn get_original_tune(title: String) -> String {
    TUNE_DB.with(|tune_db| {
        if tune_db.borrow().get(&title).is_some(){
            tune_db.borrow().get(&title).unwrap().clone()
        }
        else{
            "not found".to_string()
        }
    })
}

pub fn get_user_tune(principal: String, title: String) -> String {
    PROFILE_STORE.with(|profile_store| {
        let user_profile = profile_store.borrow().get(&principal).unwrap().clone();
        let tune = user_profile.tune_book.iter().find(|tune| tune.title == title).unwrap();
        tune.clone().tune_data.unwrap()
    })
}

pub async fn add_tune_from_origin(principal: String, title: String) -> Vec<types::Tune> {
    PROFILE_STORE.with(|profile_store| {
        let mut user_account = profile_store.borrow().get(&principal).unwrap().clone();
        let new_tune = types::Tune{
            origin: true,
            title,
            tune_data: None,
            timestamp: ic_cdk::api::time().to_string()
        };
        user_account.tune_book.push(new_tune);
        profile_store.borrow_mut().insert(principal, user_account.clone());
        user_account.tune_book
    })
}

pub async fn add_tune(principal: String, title: String, tune: String) -> Vec<types::Tune> {
    PROFILE_STORE.with(|profile_store| {
        let mut user_account = profile_store.borrow().get(&principal).unwrap().clone();
        let new_tune = types::Tune{
            origin: false,
            title,
            tune_data: Some(tune),
            timestamp: ic_cdk::api::time().to_string()
        };
        user_account.tune_book.push(new_tune);
        profile_store.borrow_mut().insert(principal, user_account.clone());
        user_account.tune_book
    })
}