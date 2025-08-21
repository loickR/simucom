use std::{error::Error, sync::{Arc, Mutex}, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::TcpStream, sync::mpsc::{self, Receiver, Sender}};

use crate::message1553::Message1553;


#[derive(Debug)]
pub struct Node {
    pub(crate) reader: ReaderMessage1553,
    pub(crate) sender: SenderMessage1553
}

impl Node {
    
    pub async fn handle_stream(address : &str, port : u16) -> Result<(ReaderMessage1553, SenderMessage1553), Box<dyn Error>> {
        let socket = match TcpStream::connect(format!("{}:{}", address, port)).await {
            Ok(co_socket) => co_socket,
            Err(_) => panic!("Unable to connect to the distant server")
        };
        
        let (half_reader , half_writer) = tokio::io::split(socket);

        Ok((ReaderMessage1553::new(half_reader), SenderMessage1553::new(half_writer)))
    }

    pub async fn handle_stream_read(reader1553 : &mut ReaderMessage1553) -> Result<(), Box<dyn Error>> {
        println!("Initialisation du thread d'écoute des messages entrants ...");
        tokio::spawn(async move {
            let _ = reader1553.handle_reading();
        });
        Ok(())
    }

    pub async fn handle_stream_write(sender1553 : &mut SenderMessage1553) -> Result<(), Box<dyn Error>> {
        println!("Initialisation du thread d'envoi des messages ...");
        tokio::spawn(async move {
            let _ = sender1553.handle_writing();
        });
        Ok(())
    }
 
    pub async fn send_message(&mut self, message : &Message1553) -> Result<(), Box<dyn Error>> {
        println!("Adding message {:?} to the queue", message);
        self.sender.send_message(message).await?;
        Ok(())
    }

    pub async fn get_liste_messages_1553(&mut self) -> Vec<Message1553> {
        let mut data: Vec<Message1553> = Vec::new();
        data.push(self.reader.read_message().await);
        return data;
    }
}

#[derive(Debug)]
pub struct SenderMessage1553 {
    socket : WriteHalf<TcpStream>,
    tx: Sender<Message1553>,
    rx: Receiver<Message1553>
}

impl SenderMessage1553 {

    pub fn new(sock : WriteHalf<TcpStream>) -> SenderMessage1553 {
        let (tx, rx) = mpsc::channel(32);
        SenderMessage1553 { 
            socket: sock,
            tx: tx,
            rx: rx
        }
    }

    pub async fn handle_writing(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Attente de message");
        SenderMessage1553::handle_writing_aux(&mut self.socket, &mut self.rx).await;
        Ok(())
    }

    async fn handle_writing_aux(socket : &mut WriteHalf<TcpStream>, rx: &mut Receiver<Message1553>) -> Result<(), Box<dyn Error>> {
        rx.try_recv().iter().clone().for_each(|msg| {
            println!("Message to send : {:?}", msg);
            let message_to_send = msg;
            socket.write(&message_to_send.do_encode());
            std::thread::sleep(Duration::from_millis(100));
        });
        
        Ok(())
    }

    pub async fn send_message(&mut self, message : &Message1553) -> Result<(), Box<dyn Error>> {
        println!("Adding message {:?} to the queue", message);
        self.socket.write(&message.do_encode());
        Ok(())
    }
}

#[derive(Debug)]
pub struct ReaderMessage1553 {
    socket : ReadHalf<TcpStream>,
    list_message_to_receive: Arc<Mutex<Vec<Message1553>>>,
    rx : Receiver<Message1553>,
    tx : Sender<Message1553>
}

impl ReaderMessage1553 {

    pub fn new(sock : ReadHalf<TcpStream>) -> ReaderMessage1553 {
        let (tx, rx) = mpsc::channel(32);
        ReaderMessage1553 { 
            socket: sock,
            list_message_to_receive: Arc::new(Mutex::new(Vec::new())),
            rx: rx,
            tx: tx
        }
    }

    pub async fn handle_reading(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut buf = Vec::new();
            self.socket.read(&mut buf);
            let message = Message1553::do_decode(&buf);
            self.tx.send(message);
        }
    }

    pub async fn read_message(&mut self) -> Message1553 {
        self.rx.recv().await.unwrap()
    }
}
