use serde::{Deserialize, Serialize};

use crate::spec_buffer::SpecBuffer;


// Structure pour représenter un message 1553
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Message1553 {
    command_word: u16,
    addr: String,
    sub_addr: String,
    size: u16,
    data_words: Vec<u16>,
}

impl Message1553 {

    pub fn new(command_word: u16, addr: String, sub_addr: String, data_words: Vec<u16>) -> Self {
        Self {
            command_word : command_word,
            addr : addr,
            sub_addr : sub_addr,
            size : data_words.len() as u16,
            data_words : data_words
        }
    }

    pub fn get_command_word(self) -> u16 {
        self.command_word
    }

    pub fn get_adresse_1553(self) -> String {
        self.addr
    }

    pub fn get_sous_adresse_1553(self) -> String {
        self.sub_addr
    }

    pub fn get_taille_message(self) -> u16 {
        self.size
    }

    pub fn get_mots_donnees(self) -> Vec<u16> {
        self.data_words
    }

    pub fn do_encode(&self) -> Vec<u8> {
        println!("{:?}", self);  

        let read_cmd =  self.clone().get_command_word();
        let read_adresse = self.clone().get_adresse_1553();
        let read_sub_adresse = self.clone().get_sous_adresse_1553();
        let read_size = self.clone().get_taille_message();

        let mut byte_buffer = SpecBuffer::new();
        byte_buffer.write_u16(read_cmd);
        byte_buffer.write_string(read_adresse);
        byte_buffer.write_string(read_sub_adresse);
        byte_buffer.write_u16(read_size);

        for (_, val) in self.clone().get_mots_donnees().iter().enumerate() {
            byte_buffer.write_u16(*val);
        }

        return byte_buffer.as_bytes().to_vec()
    }

    pub fn do_decode(array_data : &[u8]) -> Message1553 {
        let mut buffer = SpecBuffer::from_bytes(array_data);
        let buffer_size = buffer.len();
        if buffer_size < 6 {
            panic!("Not enough data to read : current size = {buffer_size} bytes");
        }

        let read_cmd =  buffer.read_u16();
        let read_adresse = buffer.read_string();
        let read_sub_adresse = buffer.read_string();
        let read_size = buffer.read_u16();

        let mut data_words : Vec<u16> = Vec::new();
        for _ in 0..read_size {
            let val = buffer.read_u16();
            data_words.push(val);
        }

        let msg_decoded = Message1553::new(read_cmd, read_adresse, read_sub_adresse, data_words);
        println!("message read = {:?}", msg_decoded);
        return msg_decoded;
    }
}

#[derive(Default, Debug, Clone)]
pub struct CoupleMessage {
    pub(crate) msg : Message1553,
    pub(crate) _address : String,
    pub(crate) _port : u32
}
