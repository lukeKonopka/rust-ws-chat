use crate::message::{MessageError, SocketMessage};
use std::cell::RefCell;
use std::rc::Rc;
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
}

#[derive(Clone)]
struct ConnectedUser {
  username: String,
  sender: Sender,
}

impl ConnectedUser {
  fn create(username: String, sender: &Sender) -> Self {
    ConnectedUser {
      username,
      sender: sender.clone(),
    }
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
    connected.register_user(connected_user);
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
    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> ws::Result<()> {
    user.send_msg(&user, msg)
  }
}
