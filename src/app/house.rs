mod room;

use crate::app::house::room::Room;

pub struct House {
    rooms: Vec<Room>,
    height: u8,
    width: u8,
}

impl House {
    pub fn build(height: u8, width: u8) -> Result<House, &'static str> {
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

    pub fn add_room(&self) {
        todo!()
    }

    pub fn remove_room(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_one_room() {}
}
