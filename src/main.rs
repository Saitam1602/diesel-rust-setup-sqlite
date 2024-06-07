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
    let connection = &mut db::establish_connection();

    // controllers::status_controller::store_status(&s);

    let new_notebook: NewNotebook = NewNotebook {
        title: "notebook 1",
        status_id: None,
    };

    controllers::notebook_controller::store_notebook(&new_notebook);
    // diesel::insert_into(statuses)
    //     .values(&new_status)
    //     .execute(connection)
    //     .expect("Error saving new status");

    let results = controllers::notebook_controller::list_notebooks();

    println!("Displaying {} status", results.len());
    for status in results {
        println!("{}", status.title);
        println!("-----------\n");
        println!("{}", status.status_id.unwrap_or(0));
    }

    // println!("Displaying status {} {}", &results.name, &results.color);
}
