use std::{error::Error, thread, time::Duration};

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

    pub async fn send_message1553(mut self, message: &Message1553) -> Result<(), Box<dyn Error>> {
        self.bus_com.send_message(message).await?;
        Ok(())
    }

    pub async fn send_messages(mut self, messages: Vec<Message1553>) {
        for (_, message) in messages.iter().enumerate() {
            self.bus_com.send_message(message).await.unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
