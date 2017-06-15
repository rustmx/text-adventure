use house::House;

#[derive(Deserialize)]
pub struct Detective {
    pub name: String,
    pub current_room: i32,
    pub house: House
}

impl Detective {
    pub fn change_room(&mut self, room_id: i32) -> Result<&mut Self, &str> {

        let house_copy = self.house.clone();

        match house_copy.get(&self.current_room) {
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
