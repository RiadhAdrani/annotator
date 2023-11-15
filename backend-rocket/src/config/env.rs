use std::env;
extern crate dotenv;

lazy_static! {
    pub static ref MONGO_URL: String = env::var("MONGO_URL").unwrap().to_string();
    pub static ref REDIS_URL: String = env::var("REDIS_URL").unwrap().to_string();
    pub static ref APP_URL: String = env::var("APP_URL").unwrap().to_string();
}
