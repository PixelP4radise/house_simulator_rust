use crate::app::{App, CurrentScreen, RoomCoordinate};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen() {
        CurrentScreen::START => render_entry_screen(frame),
        CurrentScreen::RUNNING => render_running_screen(frame, app),
        CurrentScreen::EXIT => render_exit_screen(frame),
    }
}

fn render_entry_screen(frame: &mut Frame) {
    // Render entry screen widgets
    let area = frame.size();
    frame.render_widget(
        Paragraph::new("Welcome to the House Simulator! Press Enter to start.")
            .block(Block::new().borders(Borders::ALL)),
        area,
    );
}

fn render_running_screen(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Min(5)])
        .split(frame.size());

    let top_layout = layout[0];
    let bottom_layout = layout[1];

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(top_layout);

    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(bottom_layout);

    let ticks = top_layout[0];
    let command_input = top_layout[1];
    let logs = bottom_layout[0];
    let rooms = bottom_layout[1];

    frame.render_widget(
        Paragraph::new(app.command().as_str())
            .block(Block::new().borders(Borders::ALL))
            .wrap(Wrap { trim: true }),
        command_input,
    );

    frame.render_widget(
        Paragraph::new(app.log().as_str())
            .block(Block::new().borders(Borders::ALL))
            .wrap(Wrap { trim: true }),
        logs,
    );

    if app.house().is_none() {
        frame.render_widget(
            Paragraph::new("Create a House first")
                .block(Block::new().borders(Borders::ALL))
                .wrap(Wrap { trim: true }),
            ticks,
        );
    } else {
        frame.render_widget(
            Paragraph::new(app.ticks().to_string())
                .block(Block::new().borders(Borders::ALL))
                .wrap(Wrap { trim: true }),
            ticks,
        );
    }

    match app.layout().as_str() {
        "0" => {
            frame.render_widget(Block::new().borders(Borders::ALL), rooms);
        }
        "22" => {
            let room_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(rooms);
            let top_side = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[0]);
            let right_side = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[1]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));

            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[0],
            );

            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[1],
            );

            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[0],
            );

            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 4"));

            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[1],
            );
        }
        "23" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(rooms);

            let left_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);

            let right_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);

            let left_top_description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(left_top_description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[0],
            );
            let right_top_description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(right_top_description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[0],
            );
            let left_middle_description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(left_middle_description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[1],
            );
            let right_middle_description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(right_middle_description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[1],
            );
            let left_bottom_description = app
                .get_description(RoomCoordinate(1, 3))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(left_bottom_description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[2],
            );
            let right_bottom_description = app
                .get_description(RoomCoordinate(2, 3))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(right_bottom_description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[2],
            );
        }
        "24" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(rooms);

            let left_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);

            let right_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[0],
            );
            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[0],
            );
            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[1],
            );
            let description = app
                .get_description(RoomCoordinate(1, 3))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[2],
            );
            let description = app
                .get_description(RoomCoordinate(2, 3))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[2],
            );
            let description = app
                .get_description(RoomCoordinate(1, 4))
                .unwrap_or_else(|| String::from("Room 7"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[3],
            );
            let description = app
                .get_description(RoomCoordinate(2, 4))
                .unwrap_or_else(|| String::from("Room 8"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[3],
            );
        }
        "32" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(rooms);
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[0]);
            let middle = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[1]);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[2]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[0],
            );

            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[0],
            );

            let description = app
                .get_description(RoomCoordinate(3, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[0],
            );

            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[1],
            );
            let description = app
                .get_description(RoomCoordinate(3, 2))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[1],
            );
        }
        "33" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(rooms);
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);
            let middle = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[2]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[0],
            );

            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[0],
            );

            let description = app
                .get_description(RoomCoordinate(3, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[0],
            );

            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[1],
            );
            let description = app
                .get_description(RoomCoordinate(3, 2))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[1],
            );
            let description = app
                .get_description(RoomCoordinate(1, 3))
                .unwrap_or_else(|| String::from("Room 7"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[2],
            );
            let description = app
                .get_description(RoomCoordinate(2, 3))
                .unwrap_or_else(|| String::from("Room 8"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[2],
            );
            let description = app
                .get_description(RoomCoordinate(3, 3))
                .unwrap_or_else(|| String::from("Room 9"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[2],
            );
        }
        "34" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(rooms);
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);
            let middle = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[2]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[0],
            );

            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[0],
            );

            let description = app
                .get_description(RoomCoordinate(3, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[0],
            );

            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[1],
            );
            let description = app
                .get_description(RoomCoordinate(3, 2))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[1],
            );
            let description = app
                .get_description(RoomCoordinate(1, 3))
                .unwrap_or_else(|| String::from("Room 7"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[2],
            );
            let description = app
                .get_description(RoomCoordinate(2, 3))
                .unwrap_or_else(|| String::from("Room 8"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[2],
            );
            let description = app
                .get_description(RoomCoordinate(3, 3))
                .unwrap_or_else(|| String::from("Room 9"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[2],
            );
            let description = app
                .get_description(RoomCoordinate(1, 4))
                .unwrap_or_else(|| String::from("Room 10"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[3],
            );
            let description = app
                .get_description(RoomCoordinate(2, 4))
                .unwrap_or_else(|| String::from("Room 11"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[3],
            );
            let description = app
                .get_description(RoomCoordinate(3, 4))
                .unwrap_or_else(|| String::from("Room 12"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[3],
            );
        }
        "42" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(rooms);
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[0]);
            let middle_left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[1]);
            let middle_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[2]);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(room_layout[3]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[0],
            );
            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[0],
            );
            let description = app
                .get_description(RoomCoordinate(3, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[0],
            );
            let description = app
                .get_description(RoomCoordinate(4, 1))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[0],
            );
            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[1],
            );
            let description = app
                .get_description(RoomCoordinate(3, 2))
                .unwrap_or_else(|| String::from("Room 7"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[1],
            );
            let description = app
                .get_description(RoomCoordinate(4, 2))
                .unwrap_or_else(|| String::from("Room 8"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[1],
            );
        }
        "43" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(rooms);
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);
            let middle_left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);
            let middle_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[2]);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[3]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[0],
            );
            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[0],
            );
            let description = app
                .get_description(RoomCoordinate(3, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[0],
            );
            let description = app
                .get_description(RoomCoordinate(4, 1))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[0],
            );
            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[1],
            );
            let description = app
                .get_description(RoomCoordinate(3, 2))
                .unwrap_or_else(|| String::from("Room 7"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[1],
            );
            let description = app
                .get_description(RoomCoordinate(4, 2))
                .unwrap_or_else(|| String::from("Room 8"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[1],
            );
            let description = app
                .get_description(RoomCoordinate(1, 3))
                .unwrap_or_else(|| String::from("Room 9"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[2],
            );
            let description = app
                .get_description(RoomCoordinate(2, 3))
                .unwrap_or_else(|| String::from("Room 10"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[2],
            );
            let description = app
                .get_description(RoomCoordinate(3, 3))
                .unwrap_or_else(|| String::from("Room 11"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[2],
            );
            let description = app
                .get_description(RoomCoordinate(4, 3))
                .unwrap_or_else(|| String::from("Room 12"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[2],
            );
        }
        "44" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(rooms);
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);
            let middle_left = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);
            let middle_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[2]);
            let right = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[3]);

            let description = app
                .get_description(RoomCoordinate(1, 1))
                .unwrap_or_else(|| String::from("Room 1"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[0],
            );
            let description = app
                .get_description(RoomCoordinate(2, 1))
                .unwrap_or_else(|| String::from("Room 2"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[0],
            );
            let description = app
                .get_description(RoomCoordinate(3, 1))
                .unwrap_or_else(|| String::from("Room 3"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[0],
            );
            let description = app
                .get_description(RoomCoordinate(4, 1))
                .unwrap_or_else(|| String::from("Room 4"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[0],
            );
            let description = app
                .get_description(RoomCoordinate(1, 2))
                .unwrap_or_else(|| String::from("Room 5"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[1],
            );
            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 6"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[1],
            );
            let description = app
                .get_description(RoomCoordinate(3, 2))
                .unwrap_or_else(|| String::from("Room 7"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[1],
            );
            let description = app
                .get_description(RoomCoordinate(4, 2))
                .unwrap_or_else(|| String::from("Room 8"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[1],
            );
            let description = app
                .get_description(RoomCoordinate(1, 3))
                .unwrap_or_else(|| String::from("Room 9"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[2],
            );
            let description = app
                .get_description(RoomCoordinate(2, 3))
                .unwrap_or_else(|| String::from("Room 10"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[2],
            );
            let description = app
                .get_description(RoomCoordinate(3, 3))
                .unwrap_or_else(|| String::from("Room 11"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[2],
            );
            let description = app
                .get_description(RoomCoordinate(4, 3))
                .unwrap_or_else(|| String::from("Room 12"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[2],
            );
            let description = app
                .get_description(RoomCoordinate(1, 4))
                .unwrap_or_else(|| String::from("Room 13"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left[3],
            );
            let description = app
                .get_description(RoomCoordinate(2, 4))
                .unwrap_or_else(|| String::from("Room 14"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_left[3],
            );
            let description = app
                .get_description(RoomCoordinate(3, 4))
                .unwrap_or_else(|| String::from("Room 15"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle_right[3],
            );
            let description = app
                .get_description(RoomCoordinate(4, 4))
                .unwrap_or_else(|| String::from("Room 16"));
            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right[3],
            );
        }
        _ => {}
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn render_exit_screen(frame: &mut Frame) {
    // Render exit screen widgets
    let area = frame.size();
    frame.render_widget(
        Paragraph::new("Thank you for using the House Simulator! Press Q to quit.")
            .block(Block::new().borders(Borders::ALL)),
        area,
    );
}
