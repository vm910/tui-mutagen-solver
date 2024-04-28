use crate::solver::{filter_useless_reagents, get_viable_start_reagents, priority_search};
use std::{
    error,
    sync::{mpsc::channel, Arc},
    thread,
    time::Instant,
};

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
    /// solver log
    pub solver_log: Vec<String>,
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
            solver_log: Vec::new(),
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

    pub fn find_solutions(&mut self) {
        self.solver_log.clear();
        self.solver_log
            .push("Removing useless reagents...".to_string());

        let (filtered_reagents, useless_reagents) =
            filter_useless_reagents(&self.exitus, &self.reagents);

        match useless_reagents.len() {
            0 => {
                self.solver_log
                    .push(" \u{21B3}No useless reagents found\n".to_string());
            }
            _ => {
                self.solver_log.push(format!(
                    " \u{21B3}Removed {}\n",
                    useless_reagents
                        .iter()
                        .map(|r| r.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }
        }

        self.solver_log
            .push("Finding viable start reagents...".to_string());
        let viable_starts = get_viable_start_reagents(&self.exitus, &filtered_reagents);

        match viable_starts.len() {
            0 => {
                self.solver_log
                    .push(" \u{21B3}No viable start reagents found\n".to_string());
                return;
            }
            _ => {
                self.solver_log.push(format!(
                    " \u{21B3}Viable starts {} \n",
                    viable_starts
                        .iter()
                        .map(|r| r.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }
        }

        let exitus = Arc::new(self.exitus.clone());
        let reagents = Arc::new(self.reagents.clone());

        let (sender, receiver) = channel();

        self.solver_log.push("Searching...\n".to_string());

        for start in viable_starts {
            let start = start.clone();
            let sender = sender.clone();
            let exitus = Arc::clone(&exitus);
            let reagents = Arc::clone(&reagents);

            thread::spawn(move || {
                let start_time = Instant::now();
                let path = priority_search(&exitus, &start, &reagents);
                let elapsed = start_time.elapsed();
                match sender.send((start, path, elapsed)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error sending path: {:?}\n", e);
                    }
                }
            });
        }

        drop(sender);

        self.solver_log.push("Done.\n".to_string());

        for (start, path_option, duration) in receiver {
            match path_option {
                Some(path) => {
                    self.solver_log.push(format!(
                        "Path for start {} \n \u{21B3}{} \n \u{21B3}found in {} microseconds\n",
                        start.name,
                        path.join(" -> "),
                        duration.as_micros(),
                    ));
                }
                None => {
                    self.solver_log
                        .push(format!("No path found for start {}", start.name));
                }
            }
        }
    }
}
