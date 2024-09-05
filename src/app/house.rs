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

    pub fn remove_room(&mut self, id: &str) -> Result<(), &'static str> {
        //add error catching
        match self.rooms.iter().position(|room| room.full_id() == id) {
            Some(index) => {
                self.rooms.remove(index);
                Ok(())
            }
            None => Err("the room with the specified id doesn't exist"),
        }
    }

    //not finished
    //it can receive refactoring
    //it can be organized as a table
    pub fn list_room(&self) -> String {
        let mut room_list = String::new();
        for room in &self.rooms {
            let id = room.id();
            let sensor_number = room.sensors_number().to_string();
            let processor_number = room.processors_number().to_string();
            let devices_number = room.devices_number().to_string();
            room_list.push_str(
                format!(
                "Room {id}, sensors: {sensor_number}, processors: {processor_number}, devices: {devices_number}\n"
                ).as_str()
            );
        }
        room_list
    }

    //not finished
    //not satisfied with error handling
    pub fn list_properties(&self, room_id: &str) -> Result<String, &'static str> {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => Ok(self.rooms[index].list_properties()),
            None => Err("the room with the specified id doesn't exist"),
        }
    }

    //not finished
    //error handling
    pub fn change_property_value(
        &mut self,
        room_id: String,
        property: String,
        value: i16,
    ) -> Result<(), &'static str> {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => self.rooms[index].change_property_value(property.as_str(), value),
            None => Err("the room with the specified id doesn't exist"),
        }
    }

    //not finished
    //mix this with processor and sensor
    //error handling
    pub fn add_device(&mut self, room_id: &str, device_type: &str) {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => {}
            None => {
                //the specified room doesn't exist
            }
        }
    }

    //not finished
    //error handling
    pub fn add_processor(&mut self, room_id: &str, command: String) {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => {
                self.rooms[index].add_processor(command);
            }
            None => {
                //the specified room doesn't exist
            }
        }
    }

    //not finished
    //error handling not implemented
    pub fn add_sensor(&mut self, room_id: &str, sensor_type: &str) {
        match self.rooms.iter().position(|room| room.full_id() == room_id) {
            Some(index) => {
                self.rooms[index].add_sensor(sensor_type.as_str());
            }
            None => {
                //the specified room doesn't exist
            }
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
        house.add_sensor("r0", "humidity");

        let room_list = house.list_room();

        let expected_output = "Room r0, sensors: 1, processors: 0, devices: 0\n";

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

        house.add_device("r0", "heater");
    }

    #[test]
    #[ignore]
    //should be run alone
    fn add_sensor() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        house.add_sensor("r0", "humidity");

        assert_eq!(house.rooms[0].sensors_number(), 1);
    }

    #[test]
    #[ignore]
    //should be run alone
    fn add_processor() {
        let mut house = House::build(2, 2).unwrap();

        house.add_room(1, 1).unwrap();

        house.add_processor("r0", String::from("on"));

        assert_eq!(house.rooms[0].processors_number(), 1);
    }
}
