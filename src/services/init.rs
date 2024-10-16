use super::db::Database;
use crate::models::{booking_model::Booking, dog_model::Dog, owner_model::Owner};
use mongodb::Client;
use std::env;
use std::error::Error;

pub async fn init_database() -> Result<Database, Box<dyn Error>> {
    // Attempt to read the MONGO_URI environment variable
    let uri = env::var("MONGO_URI").map_err(|_| "Can't read connection string from MONGO_URI")?;

    // Try to create a MongoDB client
    let client = Client::with_uri_str(&uri).await?;

    // Access the database
    let db = client.database("rustbackend");

    // Collections for booking, owner, and dog
    let booking = db.collection::<Booking>("booking");
    let owner = db.collection::<Owner>("owner");
    let dog = db.collection::<Dog>("dog");

    // Return the initialized Database struct
    Ok(Database {
        booking,
        owner,
        dog,
    })
}
