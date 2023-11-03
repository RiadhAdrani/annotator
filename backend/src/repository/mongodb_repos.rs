use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::User;
use mongodb::sync::{Client, Collection};

pub struct MongoRepo {
    pub user_collection: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();

        let uri = match env::var("MONGO_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        // connecting to mongodb
        let client = Client::with_uri_str(uri).unwrap();

        // creating a database
        let db = client.database("annotator");

        // initializing collections
        let col: Collection<User> = db.collection("User");

        MongoRepo {
            user_collection: col,
        }
    }
}
