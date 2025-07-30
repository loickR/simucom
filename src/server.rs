use std::{net::TcpListener, thread, time::Duration};

use crate::{handler::{handle_read_1553, Reader1553}, message1553::Message1553};

#[derive(Clone, Default)]
pub struct Server {
    address: String,
    port: u16,
    reader1553 : Reader1553
}

impl Server {

    pub fn new(ad: &str, p: u16) -> Self {
        Self {
            address : format!("{ad}"),
            port : p,
            reader1553 : Reader1553::new()
        }
    }

    pub fn start(&mut self) {
        if let Ok(obs) = TcpListener::bind(format!("{}:{}", self.address, self.port)) {
            for stream in obs.incoming() {
                if let Ok(res) = stream {
                    println!("New connection: {}", res.local_addr().unwrap());
                    let _ = res.set_nodelay(true);
                    let _ = res.set_ttl(100);
                    let value = self.reader1553.clone();
                    std::thread::spawn(move|| handle_read_1553(res, value));
                };
            }
        }
    }

    pub fn receive_message(self) -> Message1553 {
        thread::sleep(Duration::from_millis(100));
        let mut list_clone = self.reader1553.clone().get_liste_messages_1553();
        while list_clone.is_empty() {}
        return list_clone.pop().unwrap();
    }

    pub fn stop(&mut self) {
        // TODO
    }
}
