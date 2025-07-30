use std::{thread, time::Duration};

use crate::{message1553::Message1553, server::Server};

#[derive(Clone, Default)]
pub struct Abonne {
    _adresse: String,
    bus_server : Server
}

impl Abonne {

    pub fn new(adr: &str) -> Self {
        Self {
            _adresse : adr.to_string(),
            bus_server : Server::new(&("127.0.0.".to_string() + adr), 1553)
        }
    }

    pub fn demarrer(&mut self) {
        self.bus_server.start();
    }

    pub fn arreter(&mut self) {
        self.bus_server.stop();
    }

    // Fonction pour simuler la réception d'un message
    pub fn receive_message(self) -> Message1553 {
        // Simuler un délai pour la réception du message
        thread::sleep(Duration::from_micros(20));
        //return Message1553::new(0x0001, 12, 5, 2, vec![0x1234, 0x5678]);
        return self.bus_server.receive_message();
    }
}
