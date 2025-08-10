use std::{thread, time::Duration};

use crate::{functions::{read_functions_1553, Functions}, gerant::Gerant};

pub mod client;
pub mod handler;
pub mod server;
pub mod gerant;
pub mod abonne;
pub mod message1553;
pub mod functions;
pub mod spec_buffer;
pub mod node;



#[tokio::main]
async fn main() {
 
    let mut gerant = Gerant::new();
    gerant.demarer().await;

    let functions: Functions = read_functions_1553();

    send_message_1553(gerant, "1".to_string(), functions);

    thread::sleep(Duration::from_millis(10000));
}

fn send_message_1553(gerant : Gerant, command : String, functions: Functions) {
    gerant.send_message1553(&functions.call_function(&command));
}
