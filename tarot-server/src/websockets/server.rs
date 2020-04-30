use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use reqwest;
use serde_json;
use uuid::Uuid;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error, ErrorKind};

use tarot_lib::game::{events, events_data};
use tarot_lib::game::game::Game;
use tarot_lib::player::Player;

use crate::conf;


#[derive(Debug)]
struct GameData {
    sockets: Vec<Sender>,
    game: Game,
}


struct Connection {
    // shared data
    server_data: Arc<Mutex<HashMap<Uuid, GameData>>>,

    // connection-specific data
    ws: Sender,
    game_uuid: Option<Uuid>,
    // TODO player_uuid?
}


impl Handler for Connection {

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        println!("on_open(): {:?}", handshake);

        // parse url

        let path = handshake.request.resource();
        if path.starts_with("/game/play/") == false {
            return Err(Error::new(
                ErrorKind::Internal,
                format!("error path"),
            ));
        }

        let parts: Vec<&str> = path.split("/").collect();
        let game_uuid = Uuid::parse_str(parts[3]).unwrap();
        let player_uuid = Uuid::parse_str(parts[4]).unwrap();

        // get player

        let payload = reqwest::blocking::get(&format!("http://{}/api/player/get/{}", conf::HTTP_API_ADDR, player_uuid))
            .unwrap()
            .text()
            .unwrap();
        let player: Player = serde_json::from_str(&payload).unwrap();

        // register this new game

        let mut server_data = self.server_data.lock().unwrap();

        self.game_uuid = Some(game_uuid);

        // set game data if not already set
        if server_data.contains_key(&game_uuid) == false {
            let payload = reqwest::blocking::get(&format!("http://{}/api/game/get/{}", conf::HTTP_API_ADDR, game_uuid))
                .unwrap()
                .text()
                .unwrap();
            let game: Game = serde_json::from_str(&payload).unwrap();

            server_data.insert(game_uuid, GameData {
                sockets: Vec::new(),
                game: game,
            });
        }

        let game_data = server_data.get_mut(&game_uuid).unwrap();

        let is_already_a_player = game_data.sockets.contains(&self.ws);

        if is_already_a_player == false {
            game_data.sockets.push(self.ws.clone());
        }

        // send game data

        let msg_game = Message::Text(serde_json::to_string(
            &events::Event::Game(events_data::GameData {
                game: game_data.game.clone(),
            })
        ).unwrap());
        self.ws.send(msg_game);

        // broadcast connection to all, including self

        let msg_connect = Message::Text(serde_json::to_string(
            &events::Event::WsConnect(events_data::WsConnectData {
                username: player.username.clone(),
            })
        ).unwrap());
        let msg_join = Message::Text(serde_json::to_string(
            &events::Event::GameJoin(events_data::WsConnectData {
                username: player.username.clone(),
            })
        ).unwrap());

        for socket in &game_data.sockets {
            socket.send(msg_connect.clone());

            if is_already_a_player == false {
                socket.send(msg_join.clone());
            }
        }

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

        let ret = game_data.game.update(&deserialized);

        // send status ok/fail

        match ret {
            Ok(_) => {
                // send back "ok" and broadcast the msg to everyone else
                for socket in &game_data.sockets {
                    if *socket == self.ws {
                        // TODO send ok self.ws.send();
                    } else {
                        println!("-> send msg to {:?}", socket);

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

pub fn main() {
    let server_data = Arc::new(Mutex::new(
        HashMap::<Uuid, GameData>::new()
    ));

    listen(conf::WEBSOCKET_ADDR, |ws| { Connection {
        server_data: Arc::clone(&server_data),
        ws: ws,
        game_uuid: None,
    }}).unwrap();
}
