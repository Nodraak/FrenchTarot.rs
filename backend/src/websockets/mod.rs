use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

struct Server {
    ws: Sender,
    user: Option<String>,
}

impl Handler for Server {

    fn on_open(&mut self, _handshake: Handshake) -> Result<()> {
        println!("open {:?}", _handshake);
        // cookie = handshake.header("cookie")
        // match decode cookie
        //    Err => ws.close
        //    Ok => self.user = decoded
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("close - {:?} - {:?}", code, reason);

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
        println!("error: The server encountered an error: {:?}", err);
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("message {:?}", msg);


        // Echo the message back
        //self.ws.send(msg)

        Ok(())
    }
}

pub fn main(addr: &str) {
    listen(addr, |ws| { Server { ws: ws, user: None } }).unwrap();
}
