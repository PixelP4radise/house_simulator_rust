pub enum CurrentScreen {
    START,
    RUNNING,
    EXIT,
}

mod house;

use self::house::House;
use crate::app::house::ParameterNumber;
use crate::app::CurrentScreen::RUNNING;
use crate::ui::ui;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::Backend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::error::Error;

pub struct App {
    house: Option<House>,
    current_screen: CurrentScreen,
    command: String,
    currently_editing: bool,
    command_functions:
        HashMap<String, Box<dyn FnMut(&mut App, Vec<String>) -> Result<Option<String>, String>>>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            house: None,
            current_screen: CurrentScreen::START,
            command: String::new(),
            currently_editing: false,
            command_functions: HashMap::new(),
        };

        app.command_functions.reserve(24);

        app.command_functions
            .insert(String::from("hnew"), Box::new(Self::hnew));
        app.command_functions
            .insert(String::from("hrem"), Box::new(Self::hrem));
        app.command_functions
            .insert(String::from("znew"), Box::new(Self::znew));
        app.command_functions
            .insert(String::from("zrem"), Box::new(Self::zrem));
        app.command_functions
            .insert(String::from("zlist"), Box::new(Self::zlist));
        app.command_functions
            .insert(String::from("zcomp"), Box::new(Self::zcomp));
        app.command_functions
            .insert(String::from("zprops"), Box::new(Self::zprops));
        app.command_functions
            .insert(String::from("pmod"), Box::new(Self::pmod));
        app.command_functions
            .insert(String::from("cnew"), Box::new(Self::cnew));
        app.command_functions
            .insert(String::from("crem"), Box::new(Self::crem));
        app.command_functions
            .insert(String::from("rnew"), Box::new(Self::rnew));
        app.command_functions
            .insert(String::from("pchange"), Box::new(Self::pchange));
        app.command_functions
            .insert(String::from("rlist"), Box::new(Self::rrem));
        app.command_functions
            .insert(String::from("asoc"), Box::new(Self::asoc));
        app.command_functions
            .insert(String::from("disa"), Box::new(Self::dcom));
        app.command_functions
            .insert(String::from("dcom"), Box::new(Self::dcom));
        app.command_functions
            .insert(String::from("psave"), Box::new(Self::psave));
        app.command_functions
            .insert(String::from("prestore"), Box::new(Self::prestore));
        app.command_functions
            .insert(String::from("prem"), Box::new(Self::prem));
        app.command_functions
            .insert(String::from("plist"), Box::new(Self::plist));
        app.command_functions
            .insert(String::from("exec"), Box::new(Self::exec));

        app
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
                        KeyCode::Enter => match self.currently_editing {
                            true => {}
                            false => {}
                        },
                        _ => {}
                    },
                    CurrentScreen::EXIT => {}
                }
            }
        }
    }

    fn hnew(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 2 {
            return Err(String::from("Only Two arguments required"));
        }

        let height: u8 = match arguments[0].parse() {
            Ok(h) => h,
            Err(_) => return Err(String::from("Height must be a number")),
        };

        let width: u8 = match arguments[1].parse() {
            Ok(w) => w,
            Err(_) => return Err(String::from("Width must be a number")),
        };

        match House::build(height, width) {
            Ok(new_house) => {
                self.house = Some(new_house);
                Ok(None)
            }
            Err(e) => Err(format!("Failed to build House: {e}")),
        }
    }
    fn hrem(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 0 {
            return Err(String::from("hrem doesn't take any arguments"));
        }
        self.house = None;
        Ok(None)
    }
    fn znew(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 2 {
            return Err(String::from("znew requires two arguments"));
        }

        let row: u8 = match arguments[0].parse() {
            Ok(row) => row,
            Err(_) => return Err(String::from("Row must be a number")),
        };

        let collumn: u8 = match arguments[1].parse() {
            Ok(collumn) => collumn,
            Err(_) => return Err(String::from("Collumn must be a number")),
        };

        if let Some(house) = &mut self.house {
            match house.add_room(row, collumn) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!("Failed to add room: {e}")),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn zrem(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 1 {
            return Err(String::from("zrem only allows one argument"));
        }

        if let Some(house) = &mut self.house {
            let room_id = arguments[0].as_str();
            match house.remove_room(room_id) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!("Failed to remove room: {e}")),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn zlist(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 0 {
            return Err(String::from("zlist doesn't take any arguments"));
        }

        if let Some(house) = &self.house {
            Ok(Some(house.list_room()))
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn zcomp(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 1 {
            return Err(String::from("zcomp requires one parameter"));
        }

        let room_id = arguments[0].as_str();

        if let Some(house) = &self.house {
            match house.list_components(room_id) {
                Ok(feedback) => Ok(Some(feedback)),
                Err(e) => Err(format!(
                    "Failed to list components from room {room_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn zprops(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 1 {
            return Err(String::from("zprops requires one parameter"));
        }

        let room_id = arguments[0].as_str();

        if let Some(house) = &self.house {
            match house.list_properties(room_id) {
                Ok(feedback) => Ok(Some(feedback)),
                Err(e) => Err(format!(
                    "Failed to list properties from room {room_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn pmod(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("pmod requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let property_name = arguments[1].as_str();
        let property_value: i16 = match arguments[2].parse() {
            Ok(pv) => pv,
            Err(_) => return Err(String::from("Property Value must be a number")),
        };

        if let Some(house) = &mut self.house {
            match house.change_property_value(room_id, property_name, property_value) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "Failed to change property from room {room_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn cnew(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("cnew requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let component_type = arguments[1].as_str();
        let entity_or_command = arguments[2].clone();

        if let Some(house) = &mut self.house {
            match house.add_component(room_id, component_type, entity_or_command) {
                Ok(_) => Ok(None),
                Err(e) => return Err(format!("Could not add a component to room {room_id}: {e}")),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn crem(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("crem requires 3 arguments"));
        }

        let room_id = arguments[0].as_str();
        let component_type = arguments[1].as_str();
        let component_id = arguments[2].as_str();

        if let Some(house) = &mut self.house {
            match house.remove_component(room_id, component_type, component_id) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "Could not remove component from room {room_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn rnew(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() < 5 {
            return Err(String::from("rnew takes at least 5 arguments"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let rule_type = arguments[2].as_str();
        let sensor_id = arguments[3].as_str();

        let first_parameter: i16 = match arguments[4].parse() {
            Ok(v) => v,
            Err(_) => return Err(String::from("The rule parameter must be a number")),
        };

        let parameters: ParameterNumber = match arguments.get(5) {
            None => ParameterNumber::One(first_parameter),
            Some(value) => {
                let second_parameter: i16 = match value.parse() {
                    Ok(v) => v,
                    Err(_) => return Err(String::from("The rule parameter must be a number")),
                };
                ParameterNumber::Two(first_parameter, second_parameter)
            }
        };

        if let Some(house) = &mut self.house {
            match house.add_rule(room_id, processor_id, rule_type, sensor_id, parameters) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "Couldn't add a new rule on room {room_id} and processor {processor_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn pchange(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("pchange requires three arguments"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let command = arguments[2].clone();

        if let Some(house) = &mut self.house {
            match house.change_command(room_id, processor_id, command) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "Couldn't change command from processor {processor_id} in room {room_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn rlist(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 2 {
            return Err(String::from("rlist requires two parameters"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();

        if let Some(house) = &self.house {
            match house.list_rules(room_id, processor_id) {
                Ok(feedback) => Ok(Some(feedback)),
                Err(e) => Err(format!(
                    "Couldn't list rules from processor {processor_id} from room {room_id}: {e}"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn rrem(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("rrem requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let rule_id = arguments[2].as_str();

        if let Some(house) = &mut self.house {
            match house.remove_rule(room_id, processor_id, rule_id) {
                Ok(o_) => Ok(None),
                Err(e)=> Err(format!("Couldn't remove rule {rule_id} from processor {processor_id} that belongs to room {room_id}: {e}")),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn asoc(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("asoc requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let device_id = arguments[2].as_str();

        if let Some(house) = &mut self.house {
            match house.associate_device(room_id,processor_id,device_id){
                Ok(_) => Ok(None),
                Err(e) => Err(format!("Coulnd't associate device {device_id} to processor {processor_id} in room {room_id}: {e}")),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn disa(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("disa requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let device_id = arguments[2].as_str();

        if let Some(house) = &mut self.house {
            match house.remove_device_association(room_id, processor_id, device_id){
                Ok(_) => Ok(None),
                Err(e) => Err(format!("Couldn't remove association of device {device_id} from processor {processor_id} in room {room_id}: e")),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }

    fn dcom(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("dcom requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let command = arguments[2].clone();

        if let Some(house) = &mut self.house {
            match house.change_command(room_id, processor_id, command) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!("Couldn't change command from processor {processor_id} in room {room_id} to {}: {e}", arguments[2])),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn psave(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 3 {
            return Err(String::from("psave requires three parameters"));
        }

        let room_id = arguments[0].as_str();
        let processor_id = arguments[1].as_str();
        let name = arguments[2].clone();

        if let Some(house) = &mut self.house {
            match house.copy_processor(room_id, processor_id, name) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "Couldn't save processor {processor_id} from room {room_id} with name {} : {e}",
                    arguments[2]
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn prestore(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 1 {
            return Err(String::from("prestore requires one parameter"));
        }

        let name = arguments[0].as_str();

        if let Some(house) = &mut self.house {
            match house.restore_processor(name) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "It wasn't possible to restore processor {name} to it's room"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn prem(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 1 {
            return Err(String::from("prem requires one parameter"));
        }

        let name = arguments[0].as_str();

        if let Some(house) = &mut self.house {
            match house.remove_processor_memory(name) {
                Ok(_) => Ok(None),
                Err(e) => Err(format!(
                    "It wasn't possible to remove processor {name} from memory"
                )),
            }
        } else {
            Err(String::from("House needs to be built first"))
        }
    }

    fn plist(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        if arguments.len() != 0 {
            return Err(String::from("plist takes no parameters"));
        }

        if let Some(house) = &self.house {
            Ok(Some(house.list_processor_memory()))
        } else {
            Err(String::from("House needs to be built first"))
        }
    }
    fn exec(&mut self, arguments: Vec<String>) -> Result<Option<String>, String> {
        // if arguments.len() != 1 {
        //     return Err(String::from("exec requires one parameter"));
        // }
        //
        // if let Some(house) = &mut self.house {
        // } else {
        //     Err(String::from("House needs to be built first"))
        // }
        todo!()
    }
}

//argument number is checked
//argument parsing is made
//house existice is checked
//function match handling is made
