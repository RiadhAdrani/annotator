extern crate dotenv;

use crate::config::env::REDIS_URL;

extern crate redis;

lazy_static! {
    pub static ref CACHE_DB: RedisRepo = RedisRepo::init();
}

pub fn create_redis_connection() -> redis::Client {
    let client = redis::Client::open(REDIS_URL.to_string());

    return client.ok().unwrap();
}

pub struct RedisRepo {
    pub client: redis::Client,
}

impl RedisRepo {
    pub fn init() -> Self {
        let client = create_redis_connection();

        RedisRepo { client }
    }
}
