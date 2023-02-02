use serde::{Deserialize, Serialize};

use self::service::Doc;

pub mod service;

/** Counter data */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub text: String,
    pub current: bool,
}

impl Doc for Message {
    fn id(&self) -> Option<String> {
        match &self.id {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}
