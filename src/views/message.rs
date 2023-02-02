use serde_json::{json, Value};

use crate::data::Message;

pub fn message(messages: Vec<Message>) -> Value {
    let message_opt = messages.get(0);
    json!({
      "type": "text",
      "value": if let Some(message) = message_opt {
        message.text.clone()
      } else {
        "".to_string()
      },
      "style": {
        "fontWeight": "bold",
        "fontSize": 100,
      },
    })
}

pub fn message_list(messages: Vec<Message>) -> Value {
    let mut children: Vec<Value> = messages
        .iter()
        .map(|message| message_card(message))
        .collect();
    children.push(json!({
        "type": "form",
        "child": {
          "type": "container",
          "constraints": {
            "maxWidth": 200
          },
          "child": {
            "type": "textfield",
            "value": "",
            // "autofocus": children.is_empty(),
            "name": "message",
            "style": {
              "decoration": {
                "hintText": "New message"
              }
            },
            "onSubmitted": {
              "action": "newMessage"
            }
          }
        }
    }));
    json!({
        "type": "flex",
        "fillParent": true,
        "mainAxisAlignment": "center",
        "crossAxisAlignment": "center",
        "children": children
    })
}

fn message_card(message: &Message) -> Value {
    let white: u32 = 0xFFFFFFFF;
    let black: u32 = 0xFF000000;
    let black_opacity: u32 = 0xAA000000;
    json!({
        "type": "actionnable",
        "onPressed": {

        },
        "child": {
          "type": "container",
          "border": {
            "bottom": {
              "color": black,
              "width": 1,
            },
            "left": {
              "color": black,
              "width": 1,
            },
            "right": {
              "color": black,
              "width": 1,
            },
            "top": {
              "color": black,
              "width": 1,
            }
          },
          "decoration": {
            "borderRadius": {
              "bottomLeft": {
                "x": 8,
                "y": 8,
              },
              "bottomRight": {
                "x": 8,
                "y": 8,
              },
              "topLeft": {
                "x": 8,
                "y": 8,
              },
              "topRight": {
                "x": 8,
                "y": 8,
              },
            },
            "boxShadow": {
              "blurRadius": 10,
              "color": black_opacity,
             "offset": {
                "dx": 4,
                "dy": 4,
              },
            },
            "color": white
          },
          "child": {
            "type": "text",
            "value": message.text,
          }
        }
    })
}
