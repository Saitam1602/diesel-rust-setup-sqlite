use crate::{
    db::establish_connection, models::status::Status, schema::statuses::dsl, status::NewStatus,
};
use diesel::prelude::*;

pub fn list_statuses() -> Vec<Status> {
    let connection = &mut establish_connection();

    dsl::statuses
        .order_by(dsl::name)
        .load::<Status>(connection)
        .expect("Error loading statuses")
}

pub fn get_status(id: i32) -> Option<Status> {
    let connection = &mut establish_connection();

    dsl::statuses
        .filter(dsl::id.eq(id))
        .first::<Status>(connection)
        .ok()
}

pub fn store_status(status: &NewStatus) {
    let connection = &mut establish_connection();

    diesel::insert_into(dsl::statuses)
        .values(status)
        .execute(connection)
        .expect("Error saving new status");
}

pub fn update_status(id: i32, name: String, color: String) {
    let connection = &mut establish_connection();

    diesel::update(dsl::statuses.filter(dsl::id.eq(id)))
        .set((dsl::name.eq(name), dsl::color.eq(color)))
        .execute(connection)
        .expect("Error updating status");
}
