mod room;

use crate::app::house::room::Room;

pub trait DescribableItem {
    fn id(&self) -> usize; //id number
    fn full_id(&self) -> String; //id number with specialized letter
    fn name(&self) -> String; //name like Room or Processor or Heater
}

pub trait Tickable {
    fn tick(&self);
}

pub struct House {
    rooms: Vec<Room>,
    height: u8,
    width: u8,
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
            height,
            width,
        })
    }

    pub fn add_room(&mut self, row: u8, column: u8) -> Result<(), &'static str> {
        if row > self.height {
            return Err("row is higher than allowed");
        }
        if column > self.width {
            return Err("column is higher than allowed");
        }

        for room in &self.rooms {
            if room.row() == row && room.column() == column {
                return Err("a room is already occupying that position");
            }
        }

        self.rooms.push(Room::new(row, column));
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
            .collect()
            .concat()
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

    //not finished
    //error handling
    pub fn list_components_from_room(&self, room_id: &str) -> Result<String, &'static str> {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => Ok(self.rooms[index].list_components()),
            None => Err("the room with the specified id doesn't exist"),
        }
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
            "s" => Ok(self.rooms[index].remove_device(component_id)?),
            "p" => Ok(self.rooms[index].remove_device(component_id)?),
            "d" => Ok(self.rooms[index].remove_device(component_id)?),
            _ => Err("The letter of component specified doesn't match any known components"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::house::{DescribableItem, House};

    #[test]
    fn add_one_room() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        assert_eq!(1, house.rooms.len());
    }

    #[test]
    #[should_panic]
    fn adds_one_room_on_top_another() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();
        house.add_room(1, 1).unwrap();

        assert_eq!(1, house.rooms.len());
    }

    #[test]
    #[ignore]
    //should be run alone
    fn checks_rooms_id() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        assert_eq!(String::from("r0"), house.rooms.get(0).unwrap().full_id());
    }

    #[test]
    #[ignore]
    //should be run alone
    fn remove_room_by_id() {
        let mut house = House::build(2, 2).unwrap();
        house.add_room(1, 1).unwrap();
        house.remove_room("r0").unwrap();

        assert_eq!(0, house.rooms.len());
    }

    #[test]
    fn list_room() {
        let mut house = House::build(2, 2).unwrap();
        house.add_room(1, 1).unwrap();
        house
            .add_component("r0", "s", String::from("humidity"))
            .unwrap();

        let room_list = house.list_room();

        let expected_output = "Room r0, Sensors: 1, Processors: 0, Devices: 0\n";

        assert_eq!(room_list, expected_output);
    }

    #[test]
    fn list_properties() {
        let mut house = House::build(2, 2).unwrap();
        house.add_room(1, 1).unwrap();

        let properties_list = house.list_properties("r0").unwrap();

        let expected_output = concat!(
            "Humidity: 0\n",
            "Light: 0\n",
            "Radiation: 0\n",
            "Smoke: 0\n",
            "Sound: 0\n",
            "Temperature: 0\n",
            "Vibration: 0\n",
        );

        assert_eq!(properties_list, expected_output);
    }

    #[test]
    #[ignore]
    //should be run alone
    fn add_device() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        house
            .add_component("r0", "d", String::from("heater"))
            .unwrap();

        assert_eq!(house.rooms[0].devices_number(), 1);
    }

    #[test]
    #[ignore]
    //should be run alone
    fn add_sensor() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        house
            .add_component("r0", "s", String::from("humidity"))
            .unwrap();

        assert_eq!(house.rooms[0].sensors_number(), 1);
    }

    #[test]
    #[ignore]
    //should be run alone
    fn add_processor() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        house.add_component("r0", "p", String::from("on")).unwrap();

        assert_eq!(house.rooms[0].processors_number(), 1);
    }
}
