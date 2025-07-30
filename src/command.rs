use bytebuffer::ByteBuffer;

use crate::equipement::Equipement;

pub trait Command {
    fn new() -> Self;

    fn do_encode(&self) -> ByteBuffer;

    fn do_decode(&self, buf: ByteBuffer);

    fn get_id(&self) -> u32;

    fn destination(&self) -> Vec<Equipement>;

    fn add_destination(&mut self, eq: Equipement);

    fn clear_destination(&mut self);
}
