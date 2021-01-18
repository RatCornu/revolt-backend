use crate::database::*;
use mongodb::bson::to_document;
use serde::{Deserialize, Serialize};
use crate::util::result::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Channel {
    SavedMessages {
        #[serde(rename = "_id")]
        id: String,
        user: String,
    },
    DirectMessage {
        #[serde(rename = "_id")]
        id: String,
        active: bool,
        recipients: Vec<String>,
    },
    Group {
        #[serde(rename = "_id")]
        id: String,
        name: String,
        owner: String,
        description: String,
        recipients: Vec<String>,
    },
}

impl Channel {
    pub async fn save(&self) -> Result<()> {
        get_collection("channels")
            .insert_one(
                to_document(&self).map_err(|_| Error::DatabaseError {
                    operation: "to_bson",
                    with: "channel",
                })?,
                None,
            )
            .await
            .map_err(|_| Error::DatabaseError {
                operation: "insert_one",
                with: "channel",
            })?;

        Ok(())
    }
}