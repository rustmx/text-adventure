use std::vec::Vec;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Room<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub transitions: Vec<i32>
}


pub type House<'a> = HashMap<i32, Room<'a>>;


pub fn build_house<'a>() -> House<'a> {
    let mut house = House::new();

    house.insert(-1, Room {
        title: "Entrada",
        description: "Estás frente a la puerta de la casa",
        transitions: vec![0],
    });

    house.insert(0, Room {
        title: "Sala",
        description: "Estás en la sala, sentado en el sofá",
        transitions: vec![1, 2, 3],
    });

    house.insert(1, Room {
        title: "Comedor",
        description: "Estás en el comedor, sentado en la mesa",
        transitions: vec![0, 2],
    });

    house.insert(2, Room {
        title: "Cocina",
        description: "Estás en la cocina, en frente de la nevera",
        transitions: vec![0, 1],
    });

    house.insert(3, Room {
        title: "Pasillo",
        description: "Estás en el pasillo",
        transitions: vec![0, 4, 5, 6, 7],
    });

    house.insert(4, Room {
        title: "Baño Común",
        description: "Estás en el baño común, en frente del lavamanos",
        transitions: vec![3],
    });

    house.insert(5, Room {
        title: "Habitación Principal",
        description: "Estás en la sala, sentado en el sofá",
        transitions: vec![3],
    });

    house.insert(6, Room {
        title: "Sala",
        description: "Estás en la sala, sentado en el sofá",
        transitions: vec![3],
    });

    house.insert(7, Room {
        title: "Sala",
        description: "Estás en la sala, sentado en el sofá",
        transitions: vec![3],
    });

    house
}
