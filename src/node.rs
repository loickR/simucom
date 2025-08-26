use std::{error::Error, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::tcp::{OwnedReadHalf, OwnedWriteHalf}, sync::broadcast::{Receiver, Sender}};

use crate::message1553::Message1553;


#[derive(Debug)]
pub struct Node {
}

impl Node {

    
    pub async fn handle_stream_read(read_half: OwnedReadHalf, tx_copy_read : Sender<Message1553>) -> Result<(), Box<dyn Error>> {
        println!("Initialisation du thread d'écoute des messages entrants ...");
        let mut reader1553 = ReaderMessage1553::new(read_half, tx_copy_read);
        tokio::spawn(async move {
            let _ = reader1553.handle_reading().await;
        });
        Ok(())
    }

    pub async fn handle_stream_write(write_half : OwnedWriteHalf, rx_copy_send : Receiver<Message1553>) -> Result<(), Box<dyn Error>> {
        println!("Initialisation du thread d'envoi des messages ...");
        let mut sender1553 = SenderMessage1553::new(write_half, rx_copy_send);
        tokio::spawn(async move {
            let _ = sender1553.handle_writing().await;
        });
        Ok(())
    }
}

#[derive(Debug)]
pub struct SenderMessage1553 {
    socket : OwnedWriteHalf,
    rx: Receiver<Message1553>
}

impl SenderMessage1553 {

    pub fn new(sock : OwnedWriteHalf,  rx : Receiver<Message1553>) -> SenderMessage1553 {
        SenderMessage1553 { 
            socket: sock,
            rx: rx
        }
    }

    pub async fn handle_writing(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Attente de message");
        loop {
            self.rx.try_recv().iter().clone().for_each(|msg| {
                println!("Message to send : {:?}", msg);
                let message_to_send = msg;
                let _ = self.socket.write(&message_to_send.do_encode());
                println!("Message sent");
                std::thread::sleep(Duration::from_millis(100));
            });

            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

#[derive(Debug)]
pub struct ReaderMessage1553 {
    socket : OwnedReadHalf,
    tx : Sender<Message1553>
}

impl ReaderMessage1553 {

    pub fn new(sock : OwnedReadHalf, tx: Sender<Message1553>) -> ReaderMessage1553 {
        ReaderMessage1553 { 
            socket: sock,
            tx: tx
        }
    }

    pub async fn handle_reading(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut buf = Vec::new();
            let _ = self.socket.read(&mut buf).await;
            let message = Message1553::do_decode(&buf);
            self.tx.send(message);
        }
    }
}
