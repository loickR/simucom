use std::error::Error;

use crate::{message1553::Message1553, node::Node};



#[derive(Debug, Clone)]
pub struct Client {
    address: String,
    port: u16,
}

impl Client {
    
    pub fn new(addr: &str, p : u16) -> Self {
        Self {
            address: addr.to_string(),
            port: p
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut_self = self;
        mut_self.clone().await_connect(&mut_self.address, mut_self.clone().port.clone()).await?;

        Ok(())
    }

    pub async fn await_connect(self, address : &str, port : u16) -> Result<(), Box<dyn Error>> {
        let _client = Node::handle_stream(address, port).await?;
        println!("Connected to the server");
        
        Ok(())
    }

    pub fn send_message(&mut self, message : &Message1553) {
        println!("Adding message {:?} to the queue", message);
       // self.list_message_to_send.lock().unwrap().push(message.clone());
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        // return self.list_message_to_send.lock().unwrap().to_vec();
        Vec::new()
    }

    pub fn stop(self) {
    }
}
