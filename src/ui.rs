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
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
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
            let bottom_side = Layout::default()
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
                bottom_side[0],
            );

            let description = app
                .get_description(RoomCoordinate(2, 2))
                .unwrap_or_else(|| String::from("Room 4"));

            frame.render_widget(
                Paragraph::new(description)
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[1],
            );
        }
        "23" => {
            let room_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(rooms);

            let top_side = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);
            let bottom_side = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);

            frame.render_widget(
                Paragraph::new("Room 1")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 2")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 3")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[2],
            );
            frame.render_widget(
                Paragraph::new("Room 4")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 5")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 6")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[2],
            );
        }
        "24" => {
            let room_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(rooms);

            let top_side = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);
            let bottom_side = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);

            frame.render_widget(
                Paragraph::new("Room 1")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 2")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 3")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[2],
            );
            frame.render_widget(
                Paragraph::new("Room 4")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                top_side[3],
            );
            frame.render_widget(
                Paragraph::new("Room 5")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 6")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 7")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[2],
            );
            frame.render_widget(
                Paragraph::new("Room 8")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                bottom_side[3],
            );
        }
        "32" => {
            let room_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(1)])
                .split(rooms);

            let right_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[0]);

            let left_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[1]);

            frame.render_widget(
                Paragraph::new("Room 1")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 2")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 3")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 4")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 5")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[2],
            );
            frame.render_widget(
                Paragraph::new("Room 6")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[2],
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

            let right_side = Layout::default()
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

            let left_side = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(room_layout[2]);

            frame.render_widget(
                Paragraph::new("Room 1")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 2")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[0],
            );
            frame.render_widget(
                Paragraph::new("Room 3")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[0],
            );
            frame.render_widget(
                Paragraph::new("Room 4")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 5")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[1],
            );
            frame.render_widget(
                Paragraph::new("Room 6")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[1],
            );
            frame.render_widget(
                Paragraph::new("Room 7")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                right_side[2],
            );
            frame.render_widget(
                Paragraph::new("Room 8")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                middle[2],
            );
            frame.render_widget(
                Paragraph::new("Room 9")
                    .block(Block::new().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                left_side[2],
            );
        }
        "34" => {}
        "42" => {}
        "43" => {}
        "44" => {}
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
