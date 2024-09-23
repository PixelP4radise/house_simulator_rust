pub enum CurrentScreen {
    START,
    RUNNING,
    EXIT,
}

mod house;

use self::house::House;
use crate::app::CurrentScreen::RUNNING;
use crate::ui::ui;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::Backend;
use ratatui::Terminal;
use std::error::Error;

pub struct App {
    house: Option<House>,
    current_screen: CurrentScreen,
    command: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            house: None,
            current_screen: CurrentScreen::START,
            command: String::new(),
        }
    }

    pub fn get_current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|frame| ui(frame, self))?; //passar para dentro da app tambem

            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                match self.current_screen {
                    CurrentScreen::START => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        KeyCode::Enter => self.current_screen = RUNNING,
                        _ => {}
                    },
                    CurrentScreen::RUNNING => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        _ => {}
                    },
                    CurrentScreen::EXIT => {}
                }
            }
        }
    }
}
