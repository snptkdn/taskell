use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use anyhow::{Result, anyhow};
use mac_address::get_mac_address;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn hash(seed: String) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&seed);

    sha256.result_str()
}
pub fn get_mac_address_string() -> Result<String> {
    match get_mac_address()? {
        Some(mac_address) => Ok(mac_address.to_string()),
        None => {
            return Err(anyhow!("can't get mac address."));
        },
    }
}
