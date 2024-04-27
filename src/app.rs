use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum ActiveBlock {
    FileNameInput,
    ReagentOutput,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Reagent file name
    pub file_name_input: String,
    /// Reagent string
    pub reagent_string: String,
    /// Parsed reagents
    pub reagent_output: String,
    /// Edit mode
    pub edit_mode: bool,
    /// active block
    pub active_block: ActiveBlock,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            file_name_input: String::new(),
            reagent_string: String::new(),
            reagent_output: String::new(),
            active_block: ActiveBlock::FileNameInput,
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
