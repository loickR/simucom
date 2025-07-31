use std::{thread, time::Duration};

use crate::{client::Client, message1553::Message1553};

pub struct Gerant {
    _adresse: u8,
    bus_com: Client,
}

impl Gerant {

    pub fn new() -> Self {
        Self {
            _adresse: 0x05,
            bus_com : Client::new("127.0.0.1", 1553)
        }
    }

    pub async fn demarer(&mut self) {
        let _ = self.bus_com.start().await;
    }

    pub fn arreter(self) {
        self.bus_com.stop();
    }

    pub fn send_message1553(mut self, message: &Message1553) {
        let read_adresse = message.clone().get_adresse_1553();
        self.bus_com.send_message(message, read_adresse, 1553);
    }

    pub fn send_messages(mut self, messages: Vec<Message1553>) {
        for (_, message) in messages.iter().enumerate() {
            let read_adresse = message.clone().get_adresse_1553();
            self.bus_com.send_message(message, read_adresse, 1553);
            thread::sleep(Duration::from_millis(100));
        }
    }
}
