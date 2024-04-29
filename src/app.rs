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
    /// char index
    pub character_index: usize,
    /// reagents and exitus
    pub reagents_and_exitus: String,
}

impl Default for App {
    fn default() -> Self {
        let file_name_input = "reagents.txt".to_string();

        Self {
            running: true,
            file_name_input: file_name_input.clone(),
            reagents: Vec::new(),
            log_message: String::new(),
            exitus: Reagent {
                name: "".to_string(),
                atoms: Vec::new(),
                score: None,
            },
            reagents_and_exitus: String::new(),
            active_block: ActiveBlock::FileNameInput,
            status: Status::Neutral,
            edit_mode: false,
            solver_log: Vec::new(),
            character_index: file_name_input.len(),
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

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.file_name_input.chars().count())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn byte_index(&mut self) -> usize {
        self.file_name_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.file_name_input.len())
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();

        // match self.active_block {
        //     ActiveBlock::FileNameInput => {
        //         self.file_name_input.insert(index, new_char);
        //         self.move_cursor_right();
        //     }
        //     ActiveBlock::ReagentOutput => {
        //         self.reagents_and_exitus.insert(index, new_char);
        //         self.move_cursor_right();
        //     }
        // }

        self.file_name_input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self
                .file_name_input
                .chars()
                .take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.file_name_input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.file_name_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
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
            .push("Looking for viable start reagents...".to_string());
        let viable_starts = get_viable_start_reagents(&self.exitus, &filtered_reagents);

        match viable_starts.len() {
            0 => {
                self.solver_log
                    .push(" \u{21B3}No viable start reagents found\n".to_string());
                return;
            }
            _ => {
                self.solver_log.push(format!(
                    " \u{21B3}Found {} \n",
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
