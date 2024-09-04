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
}
