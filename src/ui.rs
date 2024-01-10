use ratatui::{
    prelude::{Alignment, Frame, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Clear}, symbols,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {

    let layout = Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        // TODO dynamically alter this depending on lane count chosen by user.
        .constraints([Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(20)])
        .split(f.size());


    f.render_widget(
        Paragraph::new(format!(
            "
             Press `Esc`, `Ctrl-C` or `q` to stop running \n\
            "
        ))
        .block(
            Block::default()
                .title("Cli-task-board")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center), 
        f.size()
    );
    f.render_widget(Clear, layout[1]);
    f.render_widget(
        Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("lane 1"), layout[1]);
}
