extern crate dotenv;

use crate::{
    config::env::MONGO_URL,
    models::{text_annotation_model::TextAnnotation, user_model::User},
};
use mongodb::sync::{Client, Collection};

pub struct MongoRepo {
    pub user_collection: Collection<User>,
    pub text_annotation_collection: Collection<TextAnnotation>,
}

lazy_static! {
    pub static ref DB: MongoRepo = MongoRepo::init();
}

impl MongoRepo {
    pub fn init() -> Self {
        // connecting to mongodb
        let client = Client::with_uri_str(MONGO_URL.clone()).unwrap();

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
