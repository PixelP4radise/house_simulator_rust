mod room;

use crate::app::house::room::Room;

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

    pub fn remove_room(&mut self, id: String) -> Result<(), &'static str> {
        //add error catching
        match self.rooms.iter().position(|room| room.id() == id) {
            Some(index) => {
                self.rooms.remove(index);
                Ok(())
            }
            None => Err("the room with the specified id doesn't exist"),
        }
    }

    pub fn list_room(&self) -> String {
        let mut room_list = String::new();
        for room in &self.rooms {
            let id = room.id();
            let sensor_number = room.sensors().to_string();
            let processor_number = room.processors().to_string();
            let devices_number = room.devices().to_string();
            room_list.push_str(
                format!(
                "Room {id}, sensors: {sensor_number}, processors: {processor_number}, devices: {devices_number}\n"
                ).as_str()
            );
        }
        room_list
    }

    //not finished
    //error handling not implemented
    pub fn add_sensor(&mut self, room_id: String, sensor_type: String) {
        match self.rooms.iter().position(|room| room.id() == room_id) {
            Some(index) => {
                self.rooms[index].add_sensor(sensor_type.as_str());
            }
            None => {
                //the specified room doesn't exist
            }
        }
    }

    //not finished
    //not satisfied with error handling
    pub fn list_properties(&self, room_id: String) -> Result<String, &'static str> {
        match self.rooms.iter().position(|room| room.id() == room_id) {
            Some(index) => Ok(self.rooms[index].list_properties()),
            None => Err("the room with the specified id doesn't exist"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::house::House;

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

        assert_eq!(String::from("r0"), house.rooms.get(0).unwrap().id());
    }

    #[test]
    #[ignore]
    //should be run alone
    fn remove_room_by_id() {
        let mut house = House::build(2, 2).unwrap();
        house.add_room(1, 1).unwrap();
        house.remove_room(String::from("r0")).unwrap();

        assert_eq!(0, house.rooms.len());
    }

    #[test]
    fn list_room() {
        let mut house = House::build(2, 2).unwrap();
        house.add_room(1, 1).unwrap();
        house.add_sensor(String::from("r0"), String::from("humidity"));

        let room_list = house.list_room();

        let expected_output = "Room r0, sensors: 1, processors: 0, devices: 0\n";

        assert_eq!(room_list, expected_output);
    }

    #[test]
    fn list_properties() {
        let mut house = House::build(2, 2).unwrap();
        house.add_room(1, 1).unwrap();

        let properties_list = house.list_properties(String::from("r0")).unwrap();

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
}
