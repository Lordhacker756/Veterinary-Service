use std::str::FromStr;

use super::db::Database;
use crate::models::{
    booking_model::{Booking, FullBooking},
    dog_model::Dog,
};
use futures_util::{StreamExt, TryStreamExt};
use log::error;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{InsertOneResult, UpdateResult},
};

impl Database {
    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        match self.booking.insert_one(booking).await {
            Ok(result) => Ok(result),
            Err(e) => {
                error!("Error creating booking: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn cancel_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::from_str(booking_id).expect("Error cancelling booking!")
                },
                doc! {
                    "$set": doc! {
                        "cancelled": true
                    }
                },
            )
            .await
            .ok()
            .expect("Error in cancelling booking");

        Ok(result)
    }

    pub async fn get_bookings(&self, owner_id: &str) -> Result<Vec<FullBooking>, Error> {
        let owner_object_id = ObjectId::from_str(owner_id)
            .map_err(|err| format!("Failed to parse owner ObjectId: {}", err))
            .expect("Error in parsing Owner Object");

        let filter = doc! {
            "owner": owner_object_id,
            "cancelled": false,
        };

        let mut cursor = self.booking.find(filter).await?;
        let mut bookings = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(booking) => {
                    // Fetch owner and dogs details here
                    let owner = self
                        .owner
                        .find_one(doc! { "_id": booking.owner })
                        .await?
                        .ok_or_else(|| format!("Owner not found for id: {}", booking.owner))
                        .expect("No owner found with the id provided");

                    let dogs = self
                        .dog
                        .find(doc! { "owner_id": booking.owner })
                        .await?
                        .try_collect::<Vec<Dog>>()
                        .await?;

                    bookings.push(FullBooking {
                        _id: booking._id,
                        owner,
                        dogs,
                        start_time: booking.start_time,
                        duration_in_minutes: booking.duration_in_minutes,
                        cancelled: booking.cancelled,
                    });
                }
                Err(e) => {
                    error!("Error fetching booking: {:?}", e);
                    return Err(e);
                }
            }
        }

        Ok(bookings)
    }
}
