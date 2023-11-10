use serde::{Deserialize, Serialize};

#[derive(Debugm serialize, Clone)]

pub struct Message{
    pub role: String, 
    pub content: String,
}