use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::{Color, Style};

use crate::app::ActiveBlock;

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
                        app.reagent_string.push('q');
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
        KeyCode::Char(val) => {
            if app.edit_mode {
                match &app.active_block {
                    ActiveBlock::FileNameInput => {
                        app.file_name_input.push(val);
                    }
                    ActiveBlock::ReagentOutput => {
                        app.reagent_string.push(val);
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
                if app.edit_mode {
                    app.reagent_string.pop();
                }
            }
        },
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
