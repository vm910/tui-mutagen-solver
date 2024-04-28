use crate::app::{App, AppResult, Status};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::ActiveBlock;
use crate::reagent::{load_reagents, parse_reagents};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            if !app.edit_mode {
                app.quit();
            } else {
                match &app.active_block {
                    ActiveBlock::FileNameInput => app.file_name_input.push('q'),
                    ActiveBlock::ReagentOutput => {
                        // app.reagent_string.push('q');
                    }
                }
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => match &app.active_block {
            ActiveBlock::FileNameInput => {
                app.switch_active_block(ActiveBlock::ReagentOutput);
            }
            ActiveBlock::ReagentOutput => {
                app.switch_active_block(ActiveBlock::FileNameInput);
            }
        },
        KeyCode::Down => match &app.active_block {
            ActiveBlock::FileNameInput => {
                app.switch_active_block(ActiveBlock::ReagentOutput);
            }
            ActiveBlock::ReagentOutput => {
                app.switch_active_block(ActiveBlock::FileNameInput);
            }
        },
        KeyCode::Char('r') => {
            if !app.edit_mode {
                match load_reagents(&app.file_name_input) {
                    Ok(contents) => {
                        let (exitus, reagents) = parse_reagents(&contents);
                        match exitus {
                            Some(e) => {
                                app.exitus = e;
                            }
                            None => {
                                app.status = Status::Error;
                                app.log_message =
                                    format!("No exitus found in {}", app.file_name_input);
                                return Ok(());
                            }
                        };
                        app.reagents = reagents;
                        app.status = Status::Ok;
                        app.log_message = format!("Loaded {} reagents", app.reagents.len());
                    }
                    Err(e) => {
                        app.status = Status::Error;
                        app.log_message = format!("Error loading reagents: {}", e);
                    }
                }
            } else {
                match &app.active_block {
                    ActiveBlock::FileNameInput => {
                        app.file_name_input.push('r');
                    }
                    ActiveBlock::ReagentOutput => {}
                }
            }
        }
        KeyCode::Char('s') => {
            if !app.edit_mode {
                if app.reagents.is_empty() {
                    app.solver_log.clear();
                    app.solver_log.push("No reagents loaded".to_string());
                    return Ok(());
                } else {
                    app.find_solutions();
                }
            } else {
                match &app.active_block {
                    ActiveBlock::FileNameInput => {
                        app.file_name_input.push('s');
                    }
                    ActiveBlock::ReagentOutput => {}
                }
            }
        }
        KeyCode::Char(val) => {
            if app.edit_mode {
                match &app.active_block {
                    ActiveBlock::FileNameInput => {
                        app.file_name_input.push(val);
                    }
                    ActiveBlock::ReagentOutput => {
                        // app.reagent_string.push(val);
                    }
                }
            }
        }
        KeyCode::Enter => {
            app.edit_mode = !app.edit_mode;
        }
        KeyCode::Backspace => match &app.active_block {
            ActiveBlock::FileNameInput => {
                if app.edit_mode {
                    app.file_name_input.pop();
                }
            }
            ActiveBlock::ReagentOutput => {
                // if app.edit_mode {
                //     app.reagent_string.pop();
                // }
            }
        },
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
