use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::{text_annotation_model::TextAnnotation, user_model::User};
use mongodb::sync::{Client, Collection};

pub struct MongoRepo {
    pub user_collection: Collection<User>,
    pub text_annotation_collection: Collection<TextAnnotation>,
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
        let user: Collection<User> = db.collection("User");
        let text_annotation: Collection<TextAnnotation> = db.collection("TextAnnotation");

        MongoRepo {
            user_collection: user,
            text_annotation_collection: text_annotation,
        }
    }
}
