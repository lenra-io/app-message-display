use core::time;
use std::thread;

// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::{service::Api, Message};

pub const MESSAGE_COLLECTION: &str = "message";

/** Lenra listener request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct UnknownListener {
    pub action: String,
    pub props: Option<Value>,
    pub event: Option<Value>,
    pub api: Option<Value>,
}

/** Lenra view request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum Listener {
    CurrentMessage(CurrentMessage),
    NewMessage(NewMessage),
}

impl Listener {
    pub fn handle(&self) {
        log::debug!("Listener: {:?}", self);
        match self {
            Listener::CurrentMessage(cm) => cm.handle(),
            Listener::NewMessage(nm) => nm.handle(),
        }
    }
}

/** Lenra listener request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct BaseListener {
    pub props: Option<Value>,
    pub event: Option<Value>,
    pub api: Api,
}

// #[async_trait]
trait ListenerHandler {
    /* async */
    fn handle(&self);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IDProps {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct MessageEvent {
    message: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CurrentMessage {
    pub props: IDProps,
    pub event: Option<Value>,
    pub api: Api,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct NewMessage {
    pub props: Option<Value>,
    pub event: MessageEvent,
    pub api: Api,
}

// #[async_trait]
impl ListenerHandler for CurrentMessage {
    /* async */
    fn handle(&self) {
        let mut counter: Message = self
            .api
            .get_doc(MESSAGE_COLLECTION, self.props.id.as_str())
            .unwrap();
        counter.current = true;
        counter = self.api.update_doc(MESSAGE_COLLECTION, counter).unwrap();
        thread::sleep(time::Duration::from_secs(5));
        counter.current = false;
        self.api.update_doc(MESSAGE_COLLECTION, counter).unwrap();
    }
}
// #[async_trait]
impl ListenerHandler for NewMessage {
    /* async */
    fn handle(&self) {
        self.api
            .create_doc(
                MESSAGE_COLLECTION,
                Message {
                    id: None,
                    text: self.event.message.clone(),
                    current: false,
                },
            )
            .unwrap();
    }
}
