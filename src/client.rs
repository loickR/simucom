use std::{error::Error, sync::{Arc, Mutex}};

use crate::{message1553::Message1553, node::Node};



#[derive(Debug, Clone)]
pub struct Client {
    address: String,
    port: u16,
    node: Vec<Arc<Mutex<Node>>>
}

impl Client {
    
    pub fn new(addr: &str, p : u16) -> Self {
        Self {
            address: addr.to_string(),
            port: p,
            node: Vec::new()
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let address = self.address.to_string();
        self.await_connect(&address, self.port).await?;

        Ok(())
    }

    pub async fn await_connect(&mut self, address : &str, port : u16) -> Result<(), Box<dyn Error>> {
        let _client = Node::handle_stream(address, port).await?;
        println!("Connected to the server");
        
        self.node.push(Arc::new(Mutex::new(_client)));

        Ok(())
    }

    pub fn send_message(&mut self, message : &Message1553) {
        println!("Adding message {:?} to the queue", message);
        self.node.get(0).unwrap().lock().unwrap().send_message(message);
       // self.list_message_to_send.lock().unwrap().push(message.clone());
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        return self.node.get(0).unwrap().lock().unwrap().clone().get_liste_messages_1553()
    }

    pub fn stop(self) {
    }
}
