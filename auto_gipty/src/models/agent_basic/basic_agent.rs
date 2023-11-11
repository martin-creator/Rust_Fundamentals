use crate::models::general::llm::Message;

#[derive(Debug, PartialEq)]

pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct BasicAgent {
    pub objectives: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>
}