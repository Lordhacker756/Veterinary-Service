use super::db::Database;
use crate::models::owner_model::Owner;
use log::error;
use mongodb::{error::Error, results::InsertOneResult};

impl Database {
    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        match self.owner.insert_one(owner).await {
            Ok(res) => Ok(res),
            Err(e) => {
                error!("Error creating owner: {:?}", e);
                Err(e)
            }
        }
    }
}
