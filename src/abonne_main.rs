
use crate::{abonne::Abonne, message1553::Message1553};

pub mod client;
pub mod handler;
pub mod server;
pub mod gerant;
pub mod abonne;
pub mod message1553;
pub mod functions;
pub mod spec_buffer;
pub mod node;

fn main() {
    // Exemple de message 1553
    let mut abonne = Abonne::new("1");

    abonne.demarrer();

    // Simuler la réception d'un message
    println!("Expecting message ...");

    let received_message: Message1553 = abonne.receive_message();
    println!("Received message: {:?}", received_message);
}
