#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate futures;
extern crate tokio_core;
extern crate tokio_process;

//#[cfg(test)] mod tests;

//#[macro_use] extern crate log;
extern crate daemonize;

use rocket_contrib::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

//use serde_json::{Value, Error};

use std::process::Command;

use tokio_core::reactor::Core;
use tokio_process::CommandExt;

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}

#[derive(Serialize, Deserialize)]
struct Instruction {
	id: Option<ID>,
    command: String,
    cwd: String,
    state: String
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/", format = "application/json", data = "<message>")]
fn new(message: Json<Instruction>, map: State<MessageMap>) -> Json<Value> {
    let mut hashmap = map.lock().expect("map lock.");
    let storedjson = json!({
    "command" : message.0.command,
    "cwd" : message.0.cwd,
    "state" : message.0.state
    });
    println!("{}", storedjson.to_string());
    let id = 1;
    hashmap.insert(id, storedjson.to_string());

	let patterns : &[_] = &['[', ']'];
	let command_vec = message.command.lines().map(|s| s.trim_matches(patterns)
		.split(',').map(String::from).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let patterns : &[_] = &[' '];
    println!("{:?}", command_vec[0][0].trim_matches(patterns));
    println!("{:?}", command_vec[0][1].trim_matches(patterns));
    println!("{:?}", command_vec[0][2].trim_matches(patterns));
	let mut core = Core::new().unwrap();
	//let cwd = format!("{}{}", "cd ", message.cwd);
	//println!("{:?}", format!("{}", message.cwd));

	let child = Command::new("cd")
					.arg(format!("{}", message.cwd))
					.spawn_async(&core.handle());
	let child = child.expect("failed to spawn");

	match core.run(child) {
        Ok(status) => println!("exit status: {}", status),
        Err(e) => panic!("failed to wait for exit: {}", e),
    }

	let child = Command::new(command_vec[0][0].trim_matches(patterns))
					.arg(command_vec[0][1].trim_matches(patterns))
					.arg(command_vec[0][2].trim_matches(patterns))
					.spawn_async(&core.handle());
	let child = child.expect("failed to spawn");

    match core.run(child) {
        Ok(status) => println!("exit status: {}", status),
        Err(e) => panic!("failed to wait for exit: {}", e),
    }
    
    Json(json!({
        "status": "The command executed successfully",
        "state": message.state,
    }))

}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![new])
        .catch(errors![not_found])
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main() {
    rocket().launch();
}
