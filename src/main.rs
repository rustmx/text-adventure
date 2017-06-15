#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::sync::{Arc, Mutex};

use rocket::State;
use rocket_contrib::JSON;

use serde_json::Value;

mod house;
mod player;

use house::{House, build_house};
use player::Detective;


#[derive(Serialize, Deserialize, Debug)]
struct Message {
    content: String
}


fn current_state(house: &House, detective: &Detective) -> Value {
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

            let transitions: Vec<Value> = room.transitions.iter()
                .filter_map(|t| {
                    match house.get(&t) {
                        Some(t_room) => {
                            Some(json!({
                                "id": t,
                                "title": t_room.title,
                                "description": t_room.description
                            }))
                        },
                        None => None
                    }
                })
                .collect();

            println!("Puedes indicar a qué habitación moverte usando su número identificador o usar 'x' para salir");

            json!({
                "current": {
                    "id": detective.current_room,
                    "title": room.title,
                    "description": room.description
                },
                "transitions": transitions
            })
        },
        None => json!({
            "error": "Habitación inválida"
        }),
    }
}

#[get("/me")]
fn me(house: State<House>,
      detective: State<Arc<Mutex<Detective>>>) -> JSON<Value> {
    let det = &detective.lock().unwrap();
    JSON(current_state(&house, &det))
}


#[post("/chat", data = "<message>")]
fn chat_handler(message: JSON<Message>,
                detective: State<Arc<Mutex<Detective>>>) -> JSON<Value> {

    let command = message.content.trim();

    match command.parse::<i32>() {
        Ok(room_number) => {
            let mut det = detective.lock().unwrap();
            match det.change_room(room_number) {
                Ok(_) => JSON(json!({
                    "location": "/me"
                })),
                Err(e) => JSON(json!({
                    "error": format!("{:?}", e)
                })),
            }
        },
        Err(err) => JSON(json!({
            "error": format!("{:?}", err)
        }))
    }
}

fn main() {
    let house: House = build_house();

    let detective: Detective = Detective {
        name: "test".to_owned(),
        current_room: -1,
        house: house.clone()
    };

    rocket::ignite()
        .manage(house)
        .manage(Arc::new(Mutex::new(detective)))
        .mount("/", routes![chat_handler, me])
        .launch();
}
