use diesel::prelude::*;
use dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("failed to find the environment variable");
    MysqlConnection::establish(&database_url).expect("failed to connect")
}