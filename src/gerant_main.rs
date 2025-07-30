use std::{error::Error, thread, time::Duration};

use crate::{functions::Functions, gerant::Gerant};

pub mod client;
pub mod equipement;
pub mod handler;
pub mod command;
pub mod server;
pub mod gerant;
pub mod abonne;
pub mod message1553;
pub mod functions;
pub mod spec_buffer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
 
    let mut gerant = Gerant::new();
    gerant.demarer().await;

    let functions: Functions = Functions::read_functions_1553();
    gerant.send_message1553(&functions.call_function("1"));

    thread::sleep(Duration::from_millis(100000));
    Ok(())
}
