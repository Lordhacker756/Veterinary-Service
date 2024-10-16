use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dog {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DogRequest {
    pub owner: String,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: DogRequest) -> Result<Self, Self::Error> {
        let owner_id = ObjectId::parse_str(&value.owner)
            .map_err(|err| format!("Failed to parse owner ObjectId: {}", err))?;

        Ok(Self {
            _id: ObjectId::new(),
            owner: owner_id,
            name: value.name,
            age: value.age,
            breed: value.breed,
        })
    }
}
