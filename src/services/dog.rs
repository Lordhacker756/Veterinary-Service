use super::db::Database;
use crate::models::dog_model::Dog;
use log::error;
use mongodb::{error::Error, results::InsertOneResult};

impl Database {
    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        match self.dog.insert_one(dog).await {
            Ok(res) => Ok(res),
            Err(e) => {
                error!("Error creating dog: {:?}", e);
                Err(e)
            }
        }
    }
}
