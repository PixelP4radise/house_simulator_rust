use crate::app::App;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("Bottom").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
}
