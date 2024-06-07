use controllers::status_controller::list_statuses;
use diesel::prelude::*;
use models::*;
use note::NewNote;
use notebook::NewNotebook;
use schema::statuses::dsl::*;
use status::{NewStatus, Status};

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    let new_note = NewNote {
        title: "note 1",
        text: "testo",
        notebook_id: None,
        status_id: None,
    };

    controllers::note_controller::update_note(
        1,
        "titlo new".to_string(),
        "text".to_string(),
        None,
        None,
    );

    let results = controllers::note_controller::get_note(1).unwrap();

    // println!("Displaying {} status", results.len());
    // for status in results {
    //     println!("{}", status.title);
    //     println!("-----------\n");
    //     println!("{}", status.text);
    // }

    println!("Displaying status {} {}", &results.title, &results.text);
}
