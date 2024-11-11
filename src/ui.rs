use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph}, Frame};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area());

    let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
    
    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
        .block(title_block);

    frame.render_widget(title, chunks[0]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {

    // Cut the given rectangle 'r' into 3 vertical sections
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Cut the middle section into 3 horizontal sections
    Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle piece
}