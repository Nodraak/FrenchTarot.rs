use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

use serde_json;
use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

use tarot_lib::game::{events_data, state_machine};


struct GameData {
    sockets: HashMap<Uuid, Sender>,  // player_uuid, Websocket
}


struct Server {
    ws: Sender,
    games_data: Arc<Mutex<HashMap<Uuid, GameData>>>,  // game_uuid, GameData
    user: Option<String>,
}


impl Handler for Server {

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        println!("on_open(): {:?}", handshake);

        let game_uuid = Uuid::parse_str("9495832a-b546-46b8-a085-86c330e3e121").unwrap();
        // TODO from handshake.request.path

        let player_uuid = Uuid::new_v4();
        // TODO from cookie
        // cookie = handshake.request.header("cookie")
        // match decode cookie
        //    Err => ws.close
        //    Ok => self.user = decoded

        // first connection for this game? create GameData

        let mut games_data = self.games_data.lock().unwrap();

        if games_data.contains_key(&game_uuid) == false {
            games_data.insert(game_uuid, GameData {sockets: HashMap::new()});
        }

        let game_data = games_data.get_mut(&game_uuid).unwrap();

        // broadcast our connection to every one else

        println!("-> for game {:?}, {} sockets", game_uuid, game_data.sockets.len());
        for (player_uuid, sender) in game_data.sockets.iter() {
            println!("-> broadcast to {}", player_uuid);
            let event = state_machine::Event::WsConnect(events_data::WsConnectData {
                username: "todo_username_from_server".to_string(),
            });
            let msg = Message::Text(serde_json::to_string(&event).unwrap());
            sender.send(msg);
        }

        // insert ws in relevant game data

        game_data.sockets.insert(player_uuid, self.ws.clone());

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("on_close(): {:?} - {:?}", code, reason);

        /*
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
        */
    }

    fn on_error(&mut self, err: Error) {
        println!("on_error(): The server encountered an error: {:?}", err);
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("on_message(): {:?}", msg);

        let deserialized: state_machine::Event = serde_json::from_slice(&msg.into_data()).unwrap();
        println!("-> {:?}", deserialized);

        // TODO broadcast WsConnect to all
        //self.ws.send(msg)

        Ok(())
    }
}

pub fn main(addr: &str) {
    let games_data = Arc::new(Mutex::new(HashMap::new()));

    listen(addr, |ws| { Server {
        ws: ws,
        games_data: Arc::clone(&games_data),
        user: None,
    } }).unwrap();
}
