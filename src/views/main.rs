use crate::listeners::MESSAGE_COLLECTION;
use serde_json::{json, Value};

pub fn main() -> Value {
    json!({
      "type": "flex",
      "direction": "vertical",
      "scroll": true,
      "spacing": 4,
      "crossAxisAlignment": "center",
      "children": [
        {
          "type": "flexible",
          "child":
            {
              "type": "view",
              "name": "message",
              "coll": MESSAGE_COLLECTION,
              "query": {
                "current": true
              },
            },
        },
        {
          "type": "view",
          "name": "messageList",
          "coll": MESSAGE_COLLECTION
        },
      ]
    })
}
