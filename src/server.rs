use crate::message::{MessageError, SocketMessage};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use ws::{Factory, Handler, Handshake, Message, Sender};

struct ConnectedUsers {
  list: Vec<ConnectedUser>,
}

impl ConnectedUsers {
  fn register_user(&mut self, user: ConnectedUser) {
    self.list.push(user);
  }

  fn get_user(&self, username: String) -> Option<&ConnectedUser> {
    self.list.iter().find(|user| user.username == username)
  }

  fn send(&self, to: String, message: SocketMessage) -> Result<(), MessageError> {
    let dest_user = self.get_user(to).ok_or(MessageError {
      msg: "cannot find user".into(),
    })?;
    let dest_sender = dest_user.sender.clone();
    let ws_msg = SocketMessage::into_ws_message(message)?;
    dest_sender.send(ws_msg)?;
    Ok(())
  }

  fn change_username(&mut self, id: Uuid, new_username: String) {
    let user = self.list.iter_mut().find(|user| user.id == id).unwrap();
    user.change_name(new_username);
  }
}

#[derive(Clone)]
struct ConnectedUser {
  id: Uuid,
  username: String,
  sender: Sender,
}

impl ConnectedUser {
  fn create(username: String, sender: &Sender) -> Self {
    ConnectedUser {
      id: Uuid::new_v4(),
      username,
      sender: sender.clone(),
    }
  }

  fn change_name(&mut self, new_name: String) {
    self.username = new_name;
  }
}

pub struct WsServer {
  connected_users: Rc<RefCell<ConnectedUsers>>,
}

impl Default for WsServer {
  fn default() -> Self {
    WsServer {
      connected_users: Rc::new(RefCell::new(ConnectedUsers { list: vec![] })),
    }
  }
}

impl Factory for WsServer {
  type Handler = ClientHandler;

  fn connection_made(&mut self, sender: Sender) -> ClientHandler {
    let mut connected = self
      .connected_users
      .try_borrow_mut()
      .expect("cannot borrow connected_users mutably");
    let connected_user = ConnectedUser::create("anon".into(), &sender);
    connected.register_user(connected_user.clone());
    ClientHandler::create(&self.connected_users, &connected_user)
  }
}

pub struct ClientHandler {
  connected_users: Rc<RefCell<ConnectedUsers>>,
  me: ConnectedUser,
}

impl ClientHandler {
  fn create(connected_users: &Rc<RefCell<ConnectedUsers>>, me: &ConnectedUser) -> Self {
    ClientHandler {
      connected_users: connected_users.clone(),
      me: me.clone(),
    }
  }
}

impl Handler for ClientHandler {
  fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
    self.me.sender.send(
      SocketMessage::into_ws_message(SocketMessage::me(&self.me.username))
        .ok()
        .unwrap(),
    )
  }

  fn on_message(&mut self, msg: Message) -> ws::Result<()> {
    let parsed_msg = SocketMessage::from_ws_message(msg);
    let mut connected = self
      .connected_users
      .try_borrow_mut()
      .expect("cannot borrow connected_users");
    match parsed_msg {
      Ok(socket_msg) => match socket_msg {
        SocketMessage::RequestMe => {
          connected
            .send(
              self.me.username.clone(),
              SocketMessage::me(&self.me.username),
            ).unwrap();
          Ok(())
        }
        SocketMessage::SetName { name } => {
          connected.change_username(self.me.id, name.clone());
          self.me.change_name(name.clone());
          Ok(())
        }
        _ => Ok(()),
      },
      Err(message_error) => panic!("on_message Error: {}", message_error.msg),
    }
  }
}
