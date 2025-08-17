use std::{error::Error, sync::{Arc, Mutex}};

use tokio::{io, net::TcpStream, sync::mpsc::{self, Receiver, Sender}};

use crate::message1553::{self, Message1553};


#[derive(Debug)]
pub struct Node {
    pub(crate) stream : Arc<Mutex<TcpStream>>,
    pub(crate) reader: Arc<Mutex<ReaderMessage1553>>,
    pub(crate) sender: Arc<Mutex<SenderMessage1553>>
}

impl Node {
    
    pub async fn handle_stream(address : &str, port : u16) -> Result<(Arc<Mutex<SenderMessage1553>>, Arc<Mutex<ReaderMessage1553>>), Box<dyn Error>> {
        let socket = match TcpStream::connect(format!("{}:{}", address, port)).await {
            Ok(co_socket) => co_socket,
            Err(_) => panic!("Unable to connect to the distant server")
        };
        
        let arc_mutex = Arc::new(Mutex::new(socket));

        let (tx , rx) = mpsc::channel::<Message1553>(32);

        let node = Node {
            stream: arc_mutex.clone(),
            reader: Arc::new(Mutex::new(ReaderMessage1553::new(arc_mutex.clone()))),
            sender: Arc::new(Mutex::new(SenderMessage1553::new(arc_mutex.clone(), tx, rx)))
        };

        Ok((node.sender, node.reader))
    }

    pub async fn handle_stream_read(reader1553 : ReaderMessage1553) -> Result<(), Box<dyn Error>> {
        println!("Initialisation du thread d'écoute des messages entrants ...");
        tokio::spawn(async move {
            let _ = reader1553.handle_reading();
        });
        Ok(())
    }

    pub async fn handle_stream_write(sender1553 : &mut SenderMessage1553) -> Result<(), Box<dyn Error>> {
        println!("Initialisation du thread d'envoi des messages ...");
        let _ = sender1553.handle_writing().await?;
        Ok(())
    }
 
    pub async fn send_message(&mut self, message : &Message1553) -> Result<(), Box<dyn Error>> {
        println!("Adding message {:?} to the queue", message);
        self.sender.lock().unwrap().send_message(message).await?;
        Ok(())
    }

    pub fn get_liste_messages_1553(self) -> Vec<Message1553> {
        return self.reader.lock().unwrap().clone().read_messages().lock().unwrap().to_vec();
    }

    pub fn stream(self) -> Arc<Mutex<TcpStream>> {
        return self.stream.clone();
    }
}

#[derive(Debug)]
pub struct SenderMessage1553 {
    socket : Arc<Mutex<TcpStream>>,
    tx: Sender<Message1553>,
    rx: Receiver<Message1553>
}

impl SenderMessage1553 {

    pub fn new(sock : Arc<Mutex<TcpStream>>, tx : Sender<Message1553>, rx : Receiver<Message1553>) -> SenderMessage1553 {
        SenderMessage1553 { 
            socket: sock,
            tx: tx,
            rx: rx
        }
    }

    pub async fn handle_writing(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Attente de message");
        self.handle_writing_aux().await?;
        Ok(())
    }

    async fn handle_writing_aux(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = self.rx.recv().await;
        println!("Message to send : {:?}", msg);
        self.socket.lock().unwrap().writable().await?;
        let message_to_send = msg.unwrap();
        match self.socket.lock().unwrap().try_write(&message_to_send.do_encode()) {
            Ok(_) => println!("Message {:?} sent", message_to_send),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("Error occured when posting message : {:?}", e);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        Ok(())
    }

    pub async fn send_message(&mut self, message : &Message1553) -> Result<(), Box<dyn Error>> {
        println!("Adding message {:?} to the queue", message);
        self.tx.send(message.clone()).await.unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ReaderMessage1553 {
    socket : Arc<Mutex<TcpStream>>,
    list_message_to_receive: Arc<Mutex<Vec<Message1553>>>
}

impl ReaderMessage1553 {

    pub fn new(sock : Arc<Mutex<TcpStream>>) -> ReaderMessage1553 {
        ReaderMessage1553 { 
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
                    panic!("{:?}", e);
                }
            }
        }
    }

    pub fn read_messages(self) -> Arc<Mutex<Vec<Message1553>>> {
        self.list_message_to_receive.clone()
    }
}
