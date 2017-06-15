use std::vec::Vec;
use std::collections::HashMap;


#[derive(Debug, Clone, Deserialize)]
pub struct Room {
    pub title: String,
    pub description: String,
    pub transitions: Vec<i32>
}


pub type House = HashMap<i32, Room>;


pub fn build_house() -> House {
    let mut house = House::new();

    house.insert(-1, Room {
        title: "Entrada".to_owned(),
        description: "Estás frente a la puerta de la casa".to_owned(),
        transitions: vec![0],
    });

    house.insert(0, Room {
        title: "Sala".to_owned(),
        description: "Estás en la sala, sentado en el sofá".to_owned(),
        transitions: vec![1, 2, 3],
    });

    house.insert(1, Room {
        title: "Comedor".to_owned(),
        description: "Estás en el comedor, sentado en la mesa".to_owned(),
        transitions: vec![0, 2],
    });

    house.insert(2, Room {
        title: "Cocina".to_owned(),
        description: "Estás en la cocina, en frente de la nevera".to_owned(),
        transitions: vec![0, 1],
    });

    house.insert(3, Room {
        title: "Pasillo".to_owned(),
        description: "Estás en el pasillo".to_owned(),
        transitions: vec![0, 4, 5, 6, 7],
    });

    house.insert(4, Room {
        title: "Baño Común".to_owned(),
        description: "Estás en el baño común, en frente del lavamanos".to_owned(),
        transitions: vec![3],
    });

    house.insert(5, Room {
        title: "Habitación Principal".to_owned(),
        description: "Estás en la sala, sentado en el sofá".to_owned(),
        transitions: vec![3],
    });

    house.insert(6, Room {
        title: "Sala".to_owned(),
        description: "Estás en la sala, sentado en el sofá".to_owned(),
        transitions: vec![3],
    });

    house.insert(7, Room {
        title: "Sala".to_owned(),
        description: "Estás en la sala, sentado en el sofá".to_owned(),
        transitions: vec![3],
    });

    house
}
