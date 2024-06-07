use controllers::status_controller::list_statuses;
use diesel::prelude::*;
use models::*;
use note::NewNote;
use notebook::NewNotebook;
use schema::statuses::dsl::*;
use status::{NewStatus, Status};
use tag::NewTag;

mod controllers;
mod db;
mod models;
mod schema;

fn main() {
    let new_tag = NewTag {
        name: "tag1",
        color: "yellow",
    };

    // controllers::tag_controller::store_tag(&new_tag);

    // let results = controllers::tag_controller::list_tags();

    controllers::tag_controller::update_tag(1, "newTAG".to_string(), "GREEN".to_string());

    let results = controllers::tag_controller::get_tag(1).unwrap();

    // println!("Displaying {} tags", results.len());
    // for status in results {
    //     println!("{}", status.name);
    //     println!("-----------\n");
    //     println!("{}", status.color);
    // }

    println!("Displaying status {} {}", &results.name, &results.color);
}
