#[allow(dead_code, unused_variables)]
mod database;

pub mod auth_utils;
use database::{DatabaseStatus,connect_to_database};
use auth_utils::login;
pub fn authenticate(cred: &auth_utils::models::Credentials) {
    if let DatabaseStatus::Connected = connect_to_database() {
        login(&cred);
    }
    println!("Start!");
}
