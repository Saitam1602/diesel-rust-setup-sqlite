use controllers::status_controller::list_statuses;
use diesel::prelude::*;
use models::*;
use notebook::NewNotebook;
use schema::statuses::dsl::*;
use status::{NewStatus, Status};

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    // controllers::status_controller::store_status(&s);

    let new_notebook: NewNotebook = NewNotebook {
        title: "notebook 1",
        notebook_id: None,
        status_id: None,
    };

    controllers::notebook_controller::update_notebook(1, "PIPPO".to_string(), None, None);
    // diesel::insert_into(statuses)
    //     .values(&new_status)
    //     .execute(connection)
    //     .expect("Error saving new status");

    let results = controllers::notebook_controller::get_notebook(1);

    // println!("Displaying {} status", results.len());
    // for status in results {
    //     println!("{}", status.title);
    //     println!("-----------\n");
    //     println!("{}", status.notebook_id.unwrap_or(0));
    //     println!("{}", status.status_id.unwrap_or(0));
    // }

    println!("Displaying notebook {} ", &results.unwrap().title);
}
