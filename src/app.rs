use std::error;

use crate::reagent::Reagent;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum ActiveBlock {
    FileNameInput,
    ReagentOutput,
}

#[derive(Debug)]
pub enum Status {
    Neutral,
    Ok,
    Error,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Reagent file name
    pub file_name_input: String,
    /// Parsed reagents
    pub reagents: Vec<Reagent>,
    /// Edit mode
    pub edit_mode: bool,
    /// active block
    pub active_block: ActiveBlock,
    /// exitus
    pub exitus: Reagent,
    /// log message
    pub log_message: String,
    /// status
    pub status: Status,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            file_name_input: "reagents.txt".to_string(),
            reagents: Vec::new(),
            log_message: String::new(),
            exitus: Reagent {
                name: "".to_string(),
                atoms: Vec::new(),
                score: None,
            },
            active_block: ActiveBlock::FileNameInput,
            status: Status::Neutral,
            edit_mode: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn switch_active_block(&mut self, active_block: ActiveBlock) {
        self.active_block = active_block;
    }
}
