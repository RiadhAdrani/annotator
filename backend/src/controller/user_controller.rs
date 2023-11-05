use crate::models::user_model::{UpdateUserBody, User};
use crate::repository::mongodb_repos::MongoRepo;
use mongodb::options::{FindOneAndUpdateOptions, UpdateModifications};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    options::FindOneOptions,
};
use rocket::State;

pub struct UserController {}

impl UserController {
    pub fn create(db: &State<MongoRepo>, body: User) -> Result<Option<User>, Error> {
        let new_doc = User {
            id: None,
            email: body.email,
            firstname: body.firstname,
            lastname: body.lastname,
            password: body.password,
            username: body.username,
        };

        let result = db
            .user_collection
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");

        let user = db
            .user_collection
            .find_one(doc! {"_id":result.inserted_id}, None);

        Ok(user.unwrap())
    }

    pub fn get(db: &State<MongoRepo>, id: String) -> Option<User> {
        let parsed = ObjectId::parse_str(id);

        match parsed {
            Ok(_id) => {
                let result = db
                    .user_collection
                    .find_one(doc! {"_id":_id}, FindOneOptions::default());

                result.unwrap()
            }
            Err(_) => None,
        }
    }

    pub fn update(db: &State<MongoRepo>, id: String, body: UpdateUserBody) -> Option<User> {
        // find user
        let v = ObjectId::parse_str(id);

        if v.is_err() {
            return None;
        }

        let _id = v.unwrap();

        // TODO: check if document exist

        let mut update = doc! {};

        // TODO: validate body
        // check value by value and insert 😑 :
        // Yeah I am bad with rust, how did you know ? :
        if body.firstname.is_some() {
            update.insert("firstname", body.firstname.unwrap());
        }
        if body.lastname.is_some() {
            update.insert("lastname", body.lastname.unwrap());
        }
        if body.password.is_some() {
            update.insert("password", body.password.unwrap());
        }
        if body.email.is_some() {
            update.insert("email", body.email.unwrap());
        }

        let result = db.user_collection.find_one_and_update(
            doc! {"_id":_id},
            UpdateModifications::Document(doc! {"$set": update}),
            FindOneAndUpdateOptions::builder()
                .return_document(mongodb::options::ReturnDocument::After)
                .build(),
        );

        if result.is_err() {
            return None;
        }

        result.unwrap()
    }
}
