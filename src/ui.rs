use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{ActiveBlock, App};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
        .split(frame.size());

    let reagent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(main_layout[0]);

    let file_name_input_block = Block::default()
        .title("Reagents file name")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(match &app.active_block {
            ActiveBlock::FileNameInput => {
                if app.edit_mode {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                }
            }
            _ => Style::default(),
        });

    let file_name_input = Paragraph::new(app.file_name_input.clone())
        .alignment(Alignment::Left)
        .block(file_name_input_block);

    frame.render_widget(file_name_input, reagent_layout[0]);

    let reagent_output = Block::default()
        .title("Reagents")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(match &app.active_block {
            ActiveBlock::ReagentOutput => {
                if app.edit_mode {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Green)
                }
            }
            _ => Style::default(),
        });
    frame.render_widget(reagent_output, reagent_layout[1]);

    let mode = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(mode, reagent_layout[2]);

    let solution_output = Block::default()
        .title("Solutions")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(solution_output, main_layout[1])
}
