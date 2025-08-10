use std::{io::{Read, Write}, net::TcpStream};

use bytebuffer::ByteBuffer;

use crate::{message1553::Message1553};

pub trait Observer1553 {
    fn notify_message(&mut self, msg: Message1553);
}

#[derive(Default)]
pub struct NetHandler {
    list_stream : Vec<TcpStream>,
}

impl NetHandler {

    pub fn new() -> Self {
        Self {
            list_stream:  Vec::new(),
        }
    }

    pub fn handle(&mut self, res : TcpStream) {
        self.list_stream.push(res);
    }
    
    pub fn send_bytes_message(&self, message: &[u8], adresse: &str) {
        self.send_message_from_bytes(message, adresse, 1553);
    }

    pub fn send_message_from_bytes(&self, message: &[u8], adresse: &str, port: u16) {
        println!("Boucle d'adressage");
        for (_, cl) in self.list_stream.iter().enumerate() {
            println!("{:?}", cl);
            let mut clone_cl = cl;
            let sock = clone_cl.peer_addr().unwrap();
            let p = sock.port();
            let ip = sock.ip();
            println!("clone_check_addr = {ip}:{p}");
            match ip.to_string().as_str() == adresse && p == port {
                true =>  {
                    println!("Sending message ...");
                    let _ = clone_cl.write(message);
                    println!("Message sent ...");
                },
                _ => println!("Unable to find matching address")
            }
        }
    }

    pub fn handle_read(&mut self) {
        let mut list_stream = Vec::new();
        list_stream.append(&mut self.list_stream);
        for (_, mut res) in list_stream.iter().enumerate() {
            let mut buf = Vec::new();
            loop {
                match res.read(&mut buf) {
                    Ok(size) => {
                        if size >= 4 {
                            let buf_in = ByteBuffer::from_bytes(&buf);
                            println!("Data received : {size} {:?} bytes read ...", buf_in);
                            break;
                        }
                        else {
                            continue;    
                        } 
                    },
                    Err(e) => {
                        println!("An error has occured : {e}");
                        break;
                    }
                }
            }
        }
    }

    pub fn clear_connect(&mut self) {
        for (_, m) in self.list_stream.iter().enumerate() {
            let _ = m.shutdown(std::net::Shutdown::Both);
        }

        self.list_stream.clear()
    }

    pub fn streams_connect(self) -> Vec<TcpStream> {
        self.list_stream
    }
}

#[derive(Debug, Clone, Default)]
pub struct Reader1553 {
    msg_list_1553 : Vec<Message1553>
}

impl Reader1553 {
    pub fn new() -> Self {
        Self {
            msg_list_1553 : Vec::new()
        }
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        return self.msg_list_1553.clone();
    }

    pub fn lire_dernier_message(self) -> Message1553 {
        return self.msg_list_1553.clone().pop().unwrap();
    }
}

impl Observer1553 for Reader1553 {
    fn notify_message(&mut self, msg: Message1553) {
        self.msg_list_1553.push(msg.clone());
    }
}

pub fn handle_read_1553<O: Observer1553>(mut socket_client: TcpStream, mut observer: O) {
    println!("Awaiting expect message ...");
    let mut buf = Vec::new();

    loop {
        match socket_client.read(&mut buf) {
            Ok(size) => {
                if size >= 9 {
                    println!("incoming data : size = {size} bytes");
                    let msg_decode =  Message1553::do_decode(&buf);
                    observer.notify_message(msg_decode);
                    buf.clear();
                }
            },
            Err(e) => {
                panic!("An error has occured : {e}");
            }
        }
    }
}
