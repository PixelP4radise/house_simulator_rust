use self::app::CurrentScreen::{EXIT, RUNNING, START};
use crate::app::{App, CurrentScreen};
use crate::ui::ui;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::Terminal;
use std::collections::HashMap;
use std::error::Error;
use std::io;

mod app;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    let mut command_functions: HashMap<
        String,
        Box<dyn FnMut(&mut App, Vec<String>) -> Result<Option<String>, String>>,
    > = HashMap::new();

    command_functions.reserve(22);

    command_functions.insert(String::from("hnew"), Box::new(App::hnew));
    command_functions.insert(String::from("hrem"), Box::new(App::hrem));
    command_functions.insert(String::from("znew"), Box::new(App::znew));
    command_functions.insert(String::from("zrem"), Box::new(App::zrem));
    command_functions.insert(String::from("zlist"), Box::new(App::zlist));
    command_functions.insert(String::from("zcomp"), Box::new(App::zcomp));
    command_functions.insert(String::from("zprops"), Box::new(App::zprops));
    command_functions.insert(String::from("pmod"), Box::new(App::pmod));
    command_functions.insert(String::from("cnew"), Box::new(App::cnew));
    command_functions.insert(String::from("crem"), Box::new(App::crem));
    command_functions.insert(String::from("rnew"), Box::new(App::rnew));
    command_functions.insert(String::from("pchange"), Box::new(App::pchange));
    command_functions.insert(String::from("rlist"), Box::new(App::rrem));
    command_functions.insert(String::from("asoc"), Box::new(App::asoc));
    command_functions.insert(String::from("disa"), Box::new(App::disa));
    command_functions.insert(String::from("dcom"), Box::new(App::dcom));
    command_functions.insert(String::from("psave"), Box::new(App::psave));
    command_functions.insert(String::from("prestore"), Box::new(App::prestore));
    command_functions.insert(String::from("prem"), Box::new(App::prem));
    command_functions.insert(String::from("plist"), Box::new(App::plist));
    command_functions.insert(String::from("next"), Box::new(App::next));
    command_functions.insert(String::from("advance"), Box::new(App::advance));

    run(&mut terminal, &mut app, &mut command_functions)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    command_functions: &mut HashMap<
        String,
        Box<dyn FnMut(&mut App, Vec<String>) -> Result<Option<String>, String>>,
    >,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|frame| ui(frame, app))?; //passar para dentro da app tambem

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen() {
                START => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Enter => app.set_current_screen(RUNNING),
                    _ => {}
                },
                RUNNING => match app.currently_editing() {
                    true => match key_event.code {
                        KeyCode::Char(value) => app.letter_into_command(value),
                        KeyCode::Backspace => {
                            app.pop_letter_from_command();
                        }
                        KeyCode::Enter => {
                            app.set_log(String::new());
                            app.set_currently_editing(false);
                            let mut command_and_arguments = app.command().trim().split_whitespace();
                            let command = match command_and_arguments.next() {
                                None => continue,
                                Some(value) => value,
                            };
                            let arguments: Vec<String> =
                                command_and_arguments.map(|arg| arg.to_string()).collect();

                            match command_functions.get_mut(command) {
                                Some(function) => match function(app, arguments) {
                                    Ok(Some(feedback)) => app.set_log(feedback),
                                    Ok(None) => app.set_log(String::from("Successful")),
                                    Err(e) => app.set_log(e),
                                },
                                None => app.set_log(String::from(
                                    "The command provided is not recognized",
                                )),
                            }

                            app.set_command(String::new());
                        }
                        _ => {}
                    },
                    false => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => app.set_current_screen(EXIT),
                        KeyCode::Enter => app.set_currently_editing(true),
                        _ => {}
                    },
                },
                EXIT => match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                    KeyCode::Enter => app.set_current_screen(RUNNING),
                    _ => {}
                },
            }
        }
    }
}
