use std::io::{self};

mod house;
mod player;

use house::{House, build_house};
use player::Detective;

fn main() {
    let house: House = build_house();

    let mut detective: Detective = Detective {
        name: "test",
        current_room: -1,
        house: &house
    };

    loop {

        match house.get(&detective.current_room) {
            Some(room) => {
                println!("Estás en: {} ({})", room.title, room.description);
                println!("Te puedes mover a:");
                for t in &room.transitions {
                    match house.get(&t) {
                        Some(t_room) => {
                            println!("\t- {}: {}", &t, t_room.title);
                        },
                        None => panic!("Transición inválida..."),
                    }
                }
                println!("Puedes indicar a qué habitación moverte usando su número identificador o usar 'x' para salir");
            },
            None => panic!("Habitación inválida..."),
        }

        let mut next_room_buf = String::new();

        match io::stdin().read_line(&mut next_room_buf) {
            Ok(_) => {
                let command = next_room_buf.trim();

                if command == "x" {
                    println!("Adiós!");
                    break;
                } else {
                    match command.parse::<i32>() {
                        Ok(room_number) => {
                            match detective.change_room(room_number) {
                                Ok(_) => {},
                                Err(e) => panic!("err: {}", e),
                            }
                        },
                        Err(err) => {
                            println!("Error: {:?}", err);
                        }
                    }
                }
            },
            Err(_) => panic!("Error al leer entrada..."),
        }
    }
}
