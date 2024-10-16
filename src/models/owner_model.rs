use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerRequest {
    pub name: String,
    pub email: String,
    pub country: String,
}

impl TryFrom<OwnerRequest> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn try_from(req: OwnerRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: req.name,
            email: req.email,
            country: req.country,
        })
    }
}
