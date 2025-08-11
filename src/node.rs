use std::{error::Error, sync::{Arc, Mutex}};

use tokio::{io, net::TcpStream};

use crate::message1553::{self, Message1553};


#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) stream : Arc<Mutex<TcpStream>>,
    pub(crate) reader: Reader,
    pub(crate) sender: Sender
}

impl Node {
    
    pub async fn handle_stream(address : &str, port : u16) -> Result<Node, Box<dyn Error>> {
        let socket = TcpStream::connect(format!("{}:{}", address, port)).await?;

        let arc_mutex = Arc::new(Mutex::new(socket));

        let node = Node {
            stream: arc_mutex.clone(),
            reader:  Reader::new(arc_mutex.clone()),
            sender:  Sender::new(arc_mutex.clone())
        };
        
        let result = Ok(node.clone());

        node.reader.handle_reading().await?;

        node.sender.handle_writing().await?;

        result
    }
 
    pub fn send_message(&mut self, message : &Message1553) {
        println!("Adding message {:?} to the queue", message);
        self.sender.send_message(message);
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        return self.reader.read_messages().lock().unwrap().to_vec();
    }

    pub fn stream(self) -> Arc<Mutex<TcpStream>> {
        return self.stream.clone();
    }
}

#[derive(Debug, Clone)]
pub struct Sender {
    socket : Arc<Mutex<TcpStream>>,
    list_message_to_send: Arc<Mutex<Vec<Message1553>>>
}

impl Sender {

    pub fn new(sock : Arc<Mutex<TcpStream>>) -> Sender {
        Sender { 
            socket: sock.clone(),
            list_message_to_send: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub async fn handle_writing(self) -> Result<(), Box<dyn Error>> {
        loop {
            self.socket.lock().unwrap().writable().await?;
            for msg in self.list_message_to_send.lock().unwrap().clone() {
                match self.socket.lock().unwrap().try_write(&msg.do_encode()) {
                    Ok(_) => println!("Message {:?} sent", msg),
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

            if !self.list_message_to_send.clone().lock().unwrap().is_empty() {
                self.list_message_to_send.lock().unwrap().clear();
            }
        }
    }

    pub fn send_message(&mut self, message : &Message1553) {
        println!("Adding message {:?} to the queue", message);
        self.list_message_to_send.lock().unwrap().push(message.clone());
    }
}

#[derive(Debug, Clone)]
pub struct Reader {
    socket : Arc<Mutex<TcpStream>>,
    list_message_to_receive: Arc<Mutex<Vec<Message1553>>>
}

impl Reader {

    pub fn new(sock : Arc<Mutex<TcpStream>>) -> Reader {
        Reader { 
            socket: sock.clone(),
            list_message_to_receive: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub async fn handle_reading(self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut buf = Vec::new();
            self.socket.lock().unwrap().readable().await?;
            match self.socket.lock().unwrap().try_read(&mut buf) {
                Ok(size) => {
                    if size as u16 >= message1553::MIN_SIZE_MESSAGE1553 {
                        self.list_message_to_receive.lock().unwrap().push(Message1553::do_decode(&buf));
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    pub fn read_messages(self) -> Arc<Mutex<Vec<Message1553>>> {
        self.list_message_to_receive.clone()
    }
}
