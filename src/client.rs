use std::{sync::{Arc, Mutex}, thread, time::Duration};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{handler::Reader1553, message1553::{CoupleMessage, Message1553}};


pub struct Client {
    address: String,
    port: u16,
    list_message_to_send : Vec<CoupleMessage>,
    reader1553 : Reader1553
}

impl Client {
    
    pub fn new(addr: &str, p : u16) -> Self {
        Self {
            address: addr.to_string(),
            port: p,
            list_message_to_send: Vec::new(),
            reader1553: Reader1553::new()
        }
    }

    pub async fn start(&mut self) {
        let socket = await_connect(&self.address, self.port).await;
        handle_writing_message(socket, self.get_liste_messages_1553());
    }

    pub fn send_message(&mut self, message : &Message1553, address : String, port : u32) {
        println!("Adding message {:?} to the queue", message);
        self.list_message_to_send.push(CoupleMessage { msg: message.clone(), _address: address, _port: port });
    }

    pub fn get_liste_messages_1553(&mut self) -> Vec<CoupleMessage> {
        return self.list_message_to_send.clone();
    }

    pub fn stop(self) {
    }

    pub fn receive_message(self) -> Message1553 {
        thread::sleep(Duration::from_millis(100));
        let mut list_clone = self.reader1553.get_liste_messages_1553();
        while list_clone.is_empty() {}
        return list_clone.pop().unwrap();
    }
}

pub async fn await_connect(address : &str, port : u16) -> TcpStream {
    loop {
        if let Ok(socket) = TcpStream::connect(format!("{}:{}", address, port)).await {
            let _ = socket.set_nodelay(true);
            let _ = socket.set_ttl(100);
            return socket;
        }

        println!("awaiting for connecting to the server ...");
        thread::sleep(Duration::from_millis(100));
    }
}

pub fn handle_writing_message(socket : TcpStream, list_message_to_send : Vec<CoupleMessage>) {
    let lock_socket = Arc::new(Mutex::new(socket));
    let lock_list_msg = Arc::new(Mutex::new(list_message_to_send));
    tokio::spawn(async move {
        loop {
            if !lock_list_msg.lock().unwrap().is_empty() {
                let co_msg = lock_list_msg.lock().unwrap().pop().unwrap();
                let message = co_msg.msg.clone();
                println!("Sending message {:?}", message);
                let _ = lock_socket.lock().unwrap().write(&message.do_encode());
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}
