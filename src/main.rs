use controllers::status_controller::list_statuses;
use diesel::prelude::*;
use models::*;
use schema::statuses::dsl::*;
use status::{NewStatus, Status};

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    let connection = &mut db::establish_connection();

    // controllers::status_controller::store_status(&s);
    controllers::status_controller::update_status(
        2,
        "Update stato".to_string(),
        "Rosso".to_string(),
    );

    // let new_status = NewStatus {
    //     name: "status 1",
    //     color: "indigo",
    // };

    // diesel::insert_into(statuses)
    //     .values(&new_status)
    //     .execute(connection)
    //     .expect("Error saving new status");

    let results = controllers::status_controller::list_statuses();

    println!("Displaying {} status", results.len());
    for status in results {
        println!("{}", status.name);
        println!("-----------\n");
        println!("{}", status.color);
    }

    // println!("Displaying status {} {}", &results.name, &results.color);
}
