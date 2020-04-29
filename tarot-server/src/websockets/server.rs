use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use serde_json;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error, ErrorKind};

use tarot_lib::game::{events, events_data};
use tarot_lib::game::game::Game;


struct GameData {
    sockets: Vec<Sender>,
    game: Option<Game>,
}


struct Connection {
    // shared data
    server_data: Arc<Mutex<HashMap<Uuid, GameData>>>,

    // connection-specific data
    ws: Sender,
    game_uuid: Option<Uuid>,
}


impl Handler for Connection {

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        println!("on_open(): {:?}", handshake);

        let path = handshake.request.resource();
        if (path.starts_with("/game/play/") == false) || (path.len() != 11+36) {
            return Err(Error::new(
                ErrorKind::Internal,
                format!("error path"),
            ));
        }

        let game_uuid = Uuid::parse_str(path.get(11..47).unwrap()).unwrap();

        println!("==>> cookies {:?}", handshake.request.header("cookie"));

        //let player_uuid = Uuid::new_v4();
        // TODO from cookie
        // cookie = handshake.request.header("cookie")
        // match decode cookie
        //    Err => ws.close
        //    Ok => self.user = decoded

        // register this new game

        self.game_uuid = Some(game_uuid);

        let mut server_data = self.server_data.lock().unwrap();

        // get or create the game
        let game_data = server_data.entry(game_uuid).or_insert(GameData {
            sockets: Vec::new(),
            game: None,
        });

        game_data.sockets.push(self.ws.clone());

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("on_close(): {:?} - {:?}", code, reason);

        let mut server_data = self.server_data.lock().unwrap();
        let game_data = server_data.get_mut(&self.game_uuid.unwrap()).unwrap();

        // remove player from list
        game_data.sockets.remove_item(&self.ws);

        // broadcast connection close to other players
        for socket in &game_data.sockets {
            let event = events::Event::WsDisconnect(events_data::WsConnectData {
                username: "todo_username_from_server".to_string(),
            });
            let msg = Message::Text(serde_json::to_string(&event).unwrap());
            socket.send(msg);
        }

        // if game does not have any players anymore, delete it
        if game_data.sockets.is_empty() {
            server_data.remove(&self.game_uuid.unwrap());
        }
    }

    fn on_error(&mut self, err: Error) {
        println!("on_error(): The server encountered an error: {:?}", err);
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("on_message(): {:?}", msg);

        let deserialized: events::Event = serde_json::from_slice(&msg.clone().into_data()).unwrap();
        println!("-> {:?}", deserialized);

        // find game id

        let mut server_data = self.server_data.lock().unwrap();
        let game_data = server_data.get_mut(&self.game_uuid.unwrap()).unwrap();

        // try updating the game

        let ret = game_data.game.as_mut().unwrap().update(&deserialized);

        // send status ok/fail

        match ret {
            Ok(_) => {
                // send back "ok" and broadcast the msg to everyone else
                for socket in &game_data.sockets {
                    if *socket == self.ws {
                        // TODO send ok self.ws.send();
                    } else {
                        socket.send(msg.clone());
                    }
                }
            },
            Err(val) =>  {
                // TODO send error self.ws.send();
                panic!(val); // TODO better error handling
            },
        }

        Ok(())
    }
}

pub fn main(addr: &str) {
    let server_data = Arc::new(Mutex::new(
        HashMap::<Uuid, GameData>::new()
    ));

    listen(addr, |ws| { Connection {
        server_data: Arc::clone(&server_data),
        ws: ws,
        game_uuid: None,
    }}).unwrap();
}
