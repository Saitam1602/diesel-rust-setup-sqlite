use crate::{db::establish_connection, models::tag::Tag, schema::tags::dsl, tag::NewTag};
use diesel::prelude::*;

pub fn list_tags() -> Vec<Tag> {
    let connection = &mut establish_connection();

    dsl::tags
        .order_by(dsl::name)
        .load::<Tag>(connection)
        .expect("Error loading statuses")
}

pub fn get_tag(id: i32) -> Option<Tag> {
    let connection = &mut establish_connection();

    dsl::tags
        .filter(dsl::id.eq(id))
        .first::<Tag>(connection)
        .ok()
}

pub fn store_tag(tag: &NewTag) {
    let connection = &mut establish_connection();

    diesel::insert_into(dsl::tags)
        .values(tag)
        .execute(connection)
        .expect("Error saving new tag");
}

pub fn update_tag(id: i32, name: String, color: String) {
    let connection = &mut establish_connection();

    diesel::update(dsl::tags.filter(dsl::id.eq(id)))
        .set((dsl::name.eq(name), dsl::color.eq(color)))
        .execute(connection)
        .expect("Error updating tag");
}
