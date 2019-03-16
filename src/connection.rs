use std::rc::Rc;
use ws::{Factory, Handler, Handshake, Message, Result, Sender};

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
}

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

  fn send_msg(&self, from: &ConnectedUser, msg: Message) -> Result<()> {
    let msg = msg.into_text()?;
    let message_content = format!("[{}] {}", from.username, msg);
    let message = Message::text(message_content);
    // from.send_msg(self, message.clone())?;
    self.sender.send(message)
  }
}

pub struct ConnectionFactory {
  connected_users: Rc<ConnectedUsers>,
}

impl Default for ConnectionFactory {
  fn default() -> Self {
    ConnectionFactory {
      connected_users: Rc::new(ConnectedUsers { list: vec![] }),
    }
  }
}

impl Factory for ConnectionFactory {
  type Handler = ClientHandler;

  fn connection_made(&mut self, sender: Sender) -> ClientHandler {
    // TODO: make this work with many connections
    let m = Rc::get_mut(&mut self.connected_users).expect("cannot get_mut connected_users");
    m.register_user(ConnectedUser::create("anon".into(), &sender));
    ClientHandler::create(&self.connected_users)
  }
}

pub struct ClientHandler {
  connected_users: Rc<ConnectedUsers>,
}

impl ClientHandler {
  fn create(connected_users: &Rc<ConnectedUsers>) -> Self {
    ClientHandler {
      connected_users: connected_users.clone(),
    }
  }
}

impl Handler for ClientHandler {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    let user = self
      .connected_users
      .get_user("anon".into())
      .expect("cannot find user anon");

    user.send_msg(&user, msg)
  }
}
