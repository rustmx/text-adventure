use house::House;

pub struct Detective<'a, 'b> {
    pub name: &'a str,
    pub current_room: i32,
    pub house: &'b House<'b>
}

impl<'a, 'b> Detective<'a, 'b> {
    pub fn change_room(&mut self, room_id: i32) -> Result<&mut Self, &str> {

        match self.house.get(&self.current_room) {
            Some(current_room) => {
                match current_room.transitions.iter().position(|x| x == &room_id) {
                    Some(_) => {
                        self.current_room = room_id;
                        Ok(self)
                    },
                    None => {
                        Err("can't move to that room from your current position")
                    }
                }
            },
            None => {
                Err("the current room does not exist in the house")
            },
        }
    }
}
