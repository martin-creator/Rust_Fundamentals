use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;


// solution Archictect
#[derive(Debug)]
pub struct Agent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}


impl AgentSolutionArchitect {
     pub fn new() -> Self {
        let attributes: BasicAgent = BasicAgent {
            objective: "Gathers information and design solutions for website development".to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self{attributes}
     }

// Retrive Project Scope
async fn call_project_scope(&mut self, factsheet:&mut FactSheet) -> ProjectScope {
    let msg_context: String = format!("{}")
}
}