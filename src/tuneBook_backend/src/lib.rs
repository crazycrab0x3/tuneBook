
use ic_cdk::{query, update};

mod utils;
mod types;

#[ic_cdk::update]
async fn init() {
    utils::init().await
}

#[ic_cdk::query]
fn authentication(principal: String) -> Option<types::Profile> {
    utils::authentication(principal)
}

#[ic_cdk::update]
async fn update_profile(principal: String, username: String, avatar: Vec<u8>) -> types::Profile {
    utils::update_profile(principal, username, avatar).await
}

#[ic_cdk::query]
fn get_original_tune_list(page_number: usize) -> Vec<String> {
    utils::get_original_tune_list(page_number)
}

#[ic_cdk::query]
fn get_original_tune(title: String) -> String {
    utils::get_original_tune(title)
}

#[ic_cdk::query]
fn get_user_tune_list(principal: String, page_number: usize) -> Vec<String> {
    utils::get_user_tune_list(principal, page_number)
}

#[ic_cdk::query]
fn get_user_tune(principal: String, title: String) -> String {
    utils::get_user_tune(principal, title)
}

#[ic_cdk::update]
async fn add_tune(principal: String, title: String, tune: String) -> bool {
    utils::add_tune(principal, title, tune).await
}