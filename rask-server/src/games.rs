use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

use crate::error::ServerError;
use crate::group::{Message, SendGroup};
use log::{error, info};
use rask_engine::error::EngineError;
use rask_engine::network::packet::ReadResource;
use rask_engine::resources::registry;

pub trait Game {
    fn run(self) -> Result<JoinHandle<()>, ServerError>;
}

#[allow(dead_code)]
pub struct RaskGame {
    game: (),
    group: SendGroup,
    users: Vec<User>,
    will_to_live: bool,
    res_cache: HashMap<u32, Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    sender: ws::Sender,
}

impl User {
    pub fn new(name: String, sender: ws::Sender) -> Self {
        User { name, sender }
    }
}

impl Game for RaskGame {
    fn run(self) -> Result<JoinHandle<()>, ServerError> {
        thread::Builder::new()
            .name(format!("group{}", self.group.id))
            .spawn(move || self.game_loop())
            .map_err(ServerError::GameCreation)
    }
}

const RES_PATH: &str = "res";

impl RaskGame {
    pub fn new(group: SendGroup) -> Self {
        Self {
            game: (),
            group,
            users: Vec::new(),
            will_to_live: true,
            res_cache: HashMap::new(),
        }
    }

    fn push_buffer(&mut self, buf_id: u32, user_id: usize) -> Result<(), ServerError> {
        self.users
            .get_mut(user_id)
            .ok_or(ServerError::InvalidUser(user_id))?
            .sender
            .send(ws::Message::from(
                self.res_cache
                    .get(&buf_id)
                    .ok_or_else(|| {
                        EngineError::ResourceMissing(format!(
                            "Resource {} is not loaded yet",
                            buf_id
                        ))
                    })?
                    .as_slice(),
            ))?;
        Ok(())
    }

    fn load_char(&mut self, chr: registry::CharacterInfo) -> Result<(), ServerError> {
        if self.res_cache.contains_key(&chr.id) {
            return Ok(());
        }
        self.res_cache.insert(
            chr.id,
            chr.read_from_file(RES_PATH).ok_or_else(|| {
                EngineError::ResourceMissing(format!("Failed to serialize {:?}", chr))
            })?,
        );
        Ok(())
    }

    fn load_resource(&mut self, res: registry::ResourceInfo) -> Result<(), ServerError> {
        if self.res_cache.contains_key(&res.id) {
            return Ok(());
        }
        self.res_cache.insert(
            res.id,
            res.read_from_file(RES_PATH).ok_or_else(|| {
                EngineError::ResourceMissing(format!("Failed to serialize {:?}", res))
            })?,
        );
        Ok(())
    }

    fn level_one(&mut self, uid: usize) -> Result<(), ServerError> {
        self.load_resource(registry::EMPTY)?;
        self.load_resource(registry::THIEF)?;
        self.load_char(registry::CHAR)?;
        self.push_buffer(registry::EMPTY.id, uid)?;
        self.push_buffer(registry::THIEF.id, uid)?;
        self.push_buffer(registry::CHAR.id, uid)
    }

    fn game_loop(mut self) {
        let _messages = self.get_messages();
        while self.will_to_live {
            //game.handle_events(messages);
            //game.tick();
            //let b = game.get_broadcast()
            //self.users.iter().foreach(|u| u.sender.send(b));
            let _messages = self.get_messages();
            thread::sleep(std::time::Duration::from_secs(5));
        }
        info!("thread killed itself");
    }

    fn add_user(&mut self, user: &User) {
        self.users.push(user.clone());
        if let Err(e) = self.level_one(self.users.len() - 1) {
            error!("Error during resoure distribution: {}", e);
        }
    }

    fn get_messages(&mut self) -> Vec<Message> {
        //  info!("receiver {:#?} is still alive", self.group.receiver);
        let (mut data, control): (Vec<Message>, Vec<Message>) =
            self.group.receiver.try_iter().partition(Message::is_data);
        control.iter().for_each(|x| match x {
            Message::Park => {
                data = Vec::new();
                thread::park();
            }
            Message::Kill => self.will_to_live = false,
            Message::Add(user) => self.add_user(&user),
            Message::Remove(sender) => {
                if let Some(pos) = self.users.iter().position(|x| x.sender == *sender) {
                    self.users.swap_remove(pos);
                }
            }
            _ => (),
        });
        data
    }
}
