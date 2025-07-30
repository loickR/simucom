use std::fs;

use serde::{Deserialize, Serialize};

use crate::message1553::Message1553;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Functions {
    functions: Vec<Message1553>
}

impl Functions {

    pub fn functions(self) -> Vec<Message1553> {
        self.functions.clone()
    }

    pub fn call_function(&self, command: &str) -> Message1553 {
        let list = self.functions.clone();
        for ele in list {
            if ele.clone().get_command_word().to_string() == command {
                return ele.clone();
            }
        }
    
        panic!("Unable to find a valid command")
    }

    pub fn read_functions_1553() -> Functions {
        let data = fs::read_to_string(&"messages1553.json").expect("Unable to read file");
        let functions_list: Functions = serde_json::from_str(&data).unwrap();
        return functions_list;
    }
}
