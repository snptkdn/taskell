use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

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
