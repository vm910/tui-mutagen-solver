use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{ActiveBlock, App, Status};

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

    let reagent_output_block = Block::default()
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

    let mut reagents_and_exitus = Vec::new();

    reagents_and_exitus.push(app.exitus.clone());

    for reagent in &app.reagents {
        reagents_and_exitus.push(reagent.clone());
    }

    let reagents: Vec<String> = reagents_and_exitus.iter().map(|r| r.to_string()).collect();

    let reagents = Paragraph::new(reagents.join("\n"))
        .alignment(Alignment::Left)
        .block(reagent_output_block);

    frame.render_widget(reagents, reagent_layout[1]);

    let log_block = Block::default()
        .title("Log")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let log = Paragraph::new(app.log_message.clone())
        .alignment(Alignment::Left)
        .style(match &app.status {
            Status::Error => Style::default().fg(Color::Red),
            Status::Ok => Style::default().fg(Color::Green),
            Status::Neutral => Style::default().fg(Color::White),
        })
        .block(log_block);
    frame.render_widget(log, reagent_layout[2]);

    let solution_output_block = Block::default()
        .title("Solver log")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let solution_output = Paragraph::new(app.solver_log.join("\n")).block(solution_output_block);
    frame.render_widget(solution_output, main_layout[1])
}
