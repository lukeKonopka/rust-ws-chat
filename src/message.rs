use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SocketMessage {
  Message {
    from: String,
    to: String,
    content: String,
  },
  SetName {
    name: String,
  },
  RequestMe,
  Me {
    name: String,
  },
  RequestUsers,
  Users {
    list: Vec<String>,
  },
  Error {
    err: MessageError,
  },
}

impl SocketMessage {
  pub fn from_ws_message(msg: ws::Message) -> Result<Self, MessageError> {
    let text_msg = ws::Message::into_text(msg)?;
    let socket_msg: SocketMessage = serde_json::from_str(&text_msg)?;
    Ok(socket_msg)
  }

  pub fn into_ws_message(msg: SocketMessage) -> Result<ws::Message, MessageError> {
    let text_socket_msg = serde_json::to_string(&msg)?;
    let text_msg = ws::Message::text(text_socket_msg);
    Ok(text_msg)
  }

  fn message(from: &str, to: &str, content: &str) -> Self {
    SocketMessage::Message {
      from: from.into(),
      to: to.into(),
      content: content.into(),
    }
  }

  pub fn error(msg: &str) -> Self {
    SocketMessage::Error {
      err: MessageError { msg: msg.into() },
    }
  }

  pub fn me(name: &str) -> Self {
    SocketMessage::Me { name: name.into() }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageError {
  pub msg: String,
}

impl From<ws::Error> for MessageError {
  fn from(err: ws::Error) -> Self {
    MessageError {
      msg: "ws::Error".into(),
    }
  }
}

impl From<serde_json::error::Error> for MessageError {
  fn from(err: serde_json::error::Error) -> Self {
    MessageError {
      msg: "serde_json::error::Error".into(),
    }
  }
}
