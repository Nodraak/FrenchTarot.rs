use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use serde_json;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error, ErrorKind};

use tarot_lib::game::{events, events_data};


struct GameData {
    socket2game: HashMap<Sender, Uuid>,
    game2sockets: HashMap<Uuid, Vec<Sender>>,
}


struct Connection {
    ws: Sender,
    game_data: Arc<Mutex<GameData>>,
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

        // register this new connection

        let mut game_data = self.game_data.lock().unwrap();

        game_data.socket2game.insert(self.ws.clone(), game_uuid);

        game_data.game2sockets
            .entry(game_uuid).or_insert(Vec::new())     // get or create game entry
            .push(self.ws.clone());                     // add socket

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("on_close(): {:?} - {:?}", code, reason);

        let mut game_data = self.game_data.lock().unwrap();
        let game_uuid = game_data.socket2game[&self.ws];

        for socket in game_data.game2sockets[&game_uuid].clone() {

            // remove player from list
            if socket == self.ws {
                game_data.socket2game.remove(&self.ws);

                match game_data.game2sockets.entry(game_uuid) {
                    Occupied(mut entry) => { entry.get_mut().remove_item(&self.ws); }
                    Vacant(entry) => {},
                }

                if game_data.game2sockets[&game_uuid].len() == 0 {
                    game_data.game2sockets.remove(&game_uuid);
                }
            }
            // broadcast connection close to other players
            else {
                let event = events::Event::WsDisconnect(events_data::WsConnectData {
                    username: "todo_username_from_server".to_string(),
                });
                let msg = Message::Text(serde_json::to_string(&event).unwrap());
                socket.send(msg);
            }
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

        let game_data = self.game_data.lock().unwrap();
        let game_uuid = game_data.socket2game[&self.ws];

        // update game
        // TODO - from tarot_lib?

        // send status ok/fail
        // TODO

        // broadcast the msg to everyone else, except us

        for socket in &game_data.game2sockets[&game_uuid] {
            if *socket != self.ws {
                socket.send(msg.clone());
            }
        }

        Ok(())
    }
}

pub fn main(addr: &str) {
    let games_data = Arc::new(Mutex::new(GameData {
        socket2game: HashMap::new(),
        game2sockets: HashMap::new(),
    }));

    listen(addr, |ws| { Connection {
        ws: ws,
        game_data: Arc::clone(&games_data),
    }}).unwrap();
}
