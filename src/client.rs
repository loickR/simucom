use std::{error::Error};

use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{message1553::Message1553, node::{Node, SenderMessage1553}};



#[derive(Debug)]
pub struct Client {
    address: String,
    port: u16,
    channel_sender: (Sender<Message1553>, Receiver<Message1553>)
}

impl Client {
    
    pub fn new(addr: &str, p : u16) -> Self {
        Self {
            address: addr.to_string(),
            port: p,
            channel_sender: mpsc::channel(32)
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let address = self.address.to_string();
        self.await_connect(&address, self.port).await?;
        Ok(())
    }

    pub async fn await_connect(&mut self, address : &str, port : u16) -> Result<(), Box<dyn Error>> {
        let (reader, sender) = match Node::handle_stream(address, port).await {
            Ok(socket) => {
                println!("Connected to the server");
                socket
            },
            Err(e) => panic!("Unable to connect to the server : {e}")
        };
    
        Node::handle_stream_write(&mut sender).await?;

        Node::handle_stream_read(&mut reader).await?;

        let mut data : Vec<u16> = Vec::new();
        data.push(100);
        data.push(200);
        data.push(300);
        sender.send_message(&Message1553::new(1, "127.0.0.1".to_string(), "5".to_string(), data)).await?;

        Ok(())
    }

    pub async fn send_message(&mut self, message : &Message1553) -> Result<(), Box<dyn Error>> {
        println!("Adding message {:?} to the queue", message);
        self.channel_sender.0.send(message.clone()).await?;
        Ok(())
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        return Vec::new()
    }

    pub fn stop(self) {
    }
}
