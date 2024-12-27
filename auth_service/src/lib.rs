#![allow(dead_code, unused_variables)]
pub mod database;
pub mod auth_utils;
use database::{connect_to_database, Status};
use auth_utils::{login, models::Credentials};




pub fn authenticate(cred: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(cred);
    };
}


// src/util.rs (more preferred)
// src/util/mod.rs

