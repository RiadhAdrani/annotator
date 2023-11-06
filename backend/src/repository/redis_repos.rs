use std::env;
extern crate dotenv;
use dotenv::dotenv;

extern crate redis;

lazy_static! {
    pub static ref CACHE_DB: RedisRepo = RedisRepo::init();
}

pub fn create_redis_connection() -> redis::Client {
    dotenv().ok();

    let uri = match env::var("REDIS_URL") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    let client = redis::Client::open(uri);

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
