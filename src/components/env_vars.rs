extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub fn get_env_var(key: &str) -> String {
    dotenv().ok();

    match env::var(key) {
        Ok(val) => val,
        Err(_) => panic!("Please set the {} environment variable", key),
    }
}
