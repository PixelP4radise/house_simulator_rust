mod room;

pub use self::room::ParameterNumber;
use crate::app::house::room::{Processor, Room};
use std::collections::HashMap;

pub trait DescribableItem {
    fn id(&self) -> usize; //id number
    fn full_id(&self) -> String; //id number with specialized letter
    fn name(&self) -> String; //name like Room or Processor or Heater
}

pub struct RoomCoordinate(pub u8, pub u8);

pub trait Tickable {
    fn tick(&mut self);
}

pub struct House {
    rooms: Vec<Room>,
    x_space: u8,
    y_space: u8,
    processor_memory: HashMap<String, Processor>,
    ticks: usize,
}

impl House {
    pub fn build(height: u8, width: u8) -> Result<Self, &'static str> {
        if height < 2 || height > 4 {
            return Err("the height doesn't match the requirements");
        }
        if width < 2 || width > 4 {
            return Err("the width doesn't match the requirements");
        }

        let mut rooms = Vec::new();
        rooms.reserve_exact(height as usize * width as usize);

        Ok(House {
            rooms,
            x_space: height,
            y_space: width,
            processor_memory: HashMap::new(),
            ticks: 0,
        })
    }

    pub fn height(&self) -> u8 {
        self.x_space
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }

    pub fn width(&self) -> u8 {
        self.y_space
    }

    pub fn add_room(&mut self, x_coordinate: u8, y_coordinate: u8) -> Result<(), &'static str> {
        if x_coordinate > self.x_space {
            return Err("X coordinate is higher than allowed");
        }
        if y_coordinate > self.y_space {
            return Err("Y coordinate is higher than allowed");
        }

        for room in &self.rooms {
            if room.x_coordinate() == x_coordinate && room.y_coordinate() == y_coordinate {
                return Err("a room is already occupying that position");
            }
        }

        self.rooms.push(Room::new(x_coordinate, y_coordinate));
        Ok(())
    }

    pub fn remove_room(&mut self, room_id: &str) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms.remove(index);
        Ok(())
    }

    pub fn list_room(&self) -> String {
        self.rooms
            .iter()
            .map(|room| {
                format!(
                    "{} {}, Sensors: {}, Processors: {}, Devices: {}\n",
                    room.name(),
                    room.full_id(),
                    room.sensors_number(),
                    room.processors_number(),
                    room.devices_number()
                )
            })
            .collect::<String>()
    }

    pub fn list_properties(&self, room_id: &str) -> Result<String, &'static str> {
        let index = self.find_room(room_id)?;
        Ok(self.rooms[index].list_properties())
    }

    //not finished
    //error handling
    //changing the property to a not allowed type
    pub fn change_property_value(
        &mut self,
        room_id: &str,
        property: &str,
        value: i16,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].change_property_value(property, value)
    }

    pub fn add_component(
        &mut self,
        room_id: &str,
        component_type: &str,
        entity_or_command: String,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        match component_type {
            "p" => Ok(self.rooms[index].add_processor(entity_or_command)),
            "s" => Ok(self.rooms[index].add_sensor(entity_or_command.as_str())?),
            "d" => Ok(self.rooms[index].add_device(entity_or_command.as_str())?),
            _ => Err("The letter of component specified doesn't match any known components"),
        }
    }

    pub fn list_components(&self, room_id: &str) -> Result<String, &'static str> {
        let index = self.find_room(room_id)?;
        Ok(self.rooms[index].list_components())
    }

    fn find_room(&self, room_id: &str) -> Result<usize, &'static str> {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => Ok(index),
            None => Err("The room with the specified id couldn't be found"),
        }
    }

    pub fn remove_component(
        &mut self,
        room_id: &str,
        component_type: &str,
        component_id: &str,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        match component_type {
            "s" => self.rooms[index].remove_device(component_id),
            "p" => self.rooms[index].remove_device(component_id),
            "d" => self.rooms[index].remove_device(component_id),
            _ => Err("The letter of component specified doesn't match any known components"),
        }
    }

    pub fn add_rule(
        &mut self,
        room_id: &str,
        processor_id: &str,
        rule_type: &str,
        sensor_id: &str,
        parameters: ParameterNumber,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].add_rule(processor_id, rule_type, sensor_id, parameters)
    }

    pub fn change_command_from_processor(
        &mut self,
        room_id: &str,
        processor_id: &str,
        command: String,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].change_command_from_processor(processor_id, command)
    }

    pub fn list_rules(&self, room_id: &str, processor_id: &str) -> Result<String, &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].list_rules(processor_id)
    }

    pub fn remove_rule(
        &mut self,
        room_id: &str,
        processor_id: &str,
        rule_id: &str,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].remove_rule(processor_id, rule_id)
    }

    pub fn associate_device(
        &mut self,
        room_id: &str,
        processor_id: &str,
        device_id: &str,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].associate_device(processor_id, device_id)
    }

    pub fn remove_device_association(
        &mut self,
        room_id: &str,
        processor_id: &str,
        device_id: &str,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        Ok(self.rooms[index].remove_device_association(processor_id, device_id)?)
    }

    pub fn copy_processor(
        &mut self,
        room_id: &str,
        processor_id: &str,
        name: String,
    ) -> Result<(), &'static str> {
        if self.processor_memory.contains_key(&name) {
            return Err("Processor with this name already exists");
        }

        let index = self.find_room(room_id)?;
        self.processor_memory
            .insert(name, self.rooms[index].copy_processor(processor_id)?);
        Ok(())
    }

    pub fn list_processor_memory(&self) -> String {
        self.processor_memory
            .iter()
            .map(|(key, value)| format!("{key} {} {}", value.full_id(), value.room_id()))
            .collect::<String>()
    }

    pub fn remove_processor_memory(&mut self, processor_name: &str) -> Result<(), &'static str> {
        if self.processor_memory.remove(processor_name).is_none() {
            Err("A processor with the specified name couldn't be found.")
        } else {
            Ok(())
        }
    }

    pub fn restore_processor(&mut self, processor_name: &str) -> Result<(), &'static str> {
        if let Some(processor) = self.processor_memory.get(processor_name) {
            match self.find_room(processor.room_id()) {
                Ok(index) => {
                    self.rooms[index].restore_processor(processor.clone());
                    Ok(())
                }
                Err(..) => Err("the room to whom the processor belonged no longer exists"),
            }
        } else {
            Err("the name specified couldn't be found in memory")
        }
    }

    pub fn get_description(&self, room_coordinate: RoomCoordinate) -> Option<String> {
        if self.rooms.is_empty() {
            return None;
        }
        let index = match self.rooms.iter().position(|room| {
            room.x_coordinate() == room_coordinate.0 && room.y_coordinate() == room_coordinate.1
        }) {
            Some(index) => index,
            None => return None,
        };

        Some(self.rooms[index].get_description())
    }

    pub fn change_command_from_device(
        &mut self,
        room_id: &str,
        device_id: &str,
        command: String,
    ) -> Result<(), &'static str> {
        let index = self.find_room(room_id)?;
        self.rooms[index].change_command_from_device(device_id, command)
    }
}

impl Tickable for House {
    fn tick(&mut self) {
        for room in &mut self.rooms {
            room.tick()
        }
        self.ticks += 1;
    }
}
