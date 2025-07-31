use std::{error::Error, sync::{Arc, Mutex}, thread, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::TcpStream};

use crate::message1553::{CoupleMessage, Message1553};


#[derive(Debug, Clone)]
pub struct Client {
    address: String,
    port: u16,
    list_message_to_send : Vec<CoupleMessage>,
    list_message_receive : Arc<Mutex<Vec<Message1553>>>
}

impl Client {
    
    pub fn new(addr: &str, p : u16) -> Self {
        Self {
            address: addr.to_string(),
            port: p,
            list_message_to_send: Vec::new(),
            list_message_receive: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let socket = await_connect(&self.address, self.port).await;
        let (read, write) = tokio::io::split(socket);

        
        self.clone().handle_receiving_message(read).await;

        self.clone().handle_writing_message(write).await;

        Ok(())
    }

    pub fn send_message(&mut self, message : &Message1553, address : String, port : u32) {
        println!("Adding message {:?} to the queue", message);
        self.list_message_to_send.push(CoupleMessage { msg: message.clone(), _address: address, _port: port });
    }

    pub fn get_liste_messages_1553(self) -> Vec<CoupleMessage> {
        return Arc::new(Mutex::new(self.list_message_to_send)).lock().unwrap().to_vec();
    }

    pub async fn handle_receiving_message(self, socket : ReadHalf<TcpStream>) {
        let lock_socket: Arc<Mutex<ReadHalf<TcpStream>>> = Arc::new(Mutex::new(socket));
        loop {
            let mut buffer = Vec::new();
            match lock_socket.lock().unwrap().read_buf(&mut buffer).await {
                Ok(size) => {
                    if size > 0 {
                        let msg = Message1553::do_decode(&buffer);
                        println!("message received : {:?}", msg);
                        self.list_message_receive.try_lock().unwrap().push(msg);
                    }
                },
                Err(_) => panic!("No can be read")
            };
        }
    }

    pub async fn handle_writing_message(self, socket : WriteHalf<TcpStream>) {
        let lock_socket: Arc<Mutex<WriteHalf<TcpStream>>> = Arc::new(Mutex::new(socket));
        loop {
            println!("Handling fifo");
            let lock_list_msg: Arc<Mutex<Vec<CoupleMessage>>> = Arc::new(Mutex::new(self.clone().get_liste_messages_1553()));
            println!("Check if any data has been pushed into the queue");
            if !lock_list_msg.lock().unwrap().is_empty() {
                println!("Fifo is not empty");
                let co_msg = lock_list_msg.lock().unwrap().pop().unwrap();
                let message = co_msg.msg.clone();
                println!("Sending message {:?}", message);
                let _ = lock_socket.lock().unwrap().write(&message.do_encode());
            }
            thread::sleep(Duration::from_millis(1000));
        }
    }

    pub fn stop(self) {
    }

    pub fn receive_message(self) -> Message1553 {
        self.list_message_receive.try_lock().unwrap().pop().unwrap()
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


