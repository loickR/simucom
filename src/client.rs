use std::error::Error;

use tokio::{net::TcpStream, sync::broadcast::{self, Receiver, Sender}};

use crate::{message1553::Message1553, node::Node};



#[derive(Debug)]
pub struct Client {
    address: String,
    port: u16,
    channel_sender: (Sender<Message1553>, Receiver<Message1553>),
    channel_reader: (Sender<Message1553>, Receiver<Message1553>)
}

impl Client {
    
    pub fn new(addr: &str, p : u16) -> Self {
        Self {
            address: addr.to_string(),
            port: p,
            channel_sender: broadcast::channel(32),
            channel_reader: broadcast::channel(32)
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let address = self.address.to_string();
        self.await_connect(&address, self.port).await?;
        Ok(())
    }

    pub async fn await_connect(&mut self, address : &str, port : u16) -> Result<(), Box<dyn Error>> {
        let (read_half, write_half) = match TcpStream::connect(format!("{}:{}", address, port)).await {
            Ok(socket) => {
                println!("Connected to the server : {:?}", socket);
                socket.into_split()
            },
            Err(_) => panic!("Unable to connect to the distant server")
        }; 

        //Node::handle_stream_read(read_half).await?;
        self.channel_sender.0 = Node::handle_stream_write(write_half).await.unwrap();

        Ok(())
    }

    pub async fn send_message(&mut self, message : &Message1553) -> Result<(), Box<dyn Error>> {
        println!("Adding message {:?} to the queue", message);
        let _ = self.channel_sender.0.send(message.clone());
        Ok(())
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        return Vec::new()
    }

    pub fn stop(self) {
    }
}
