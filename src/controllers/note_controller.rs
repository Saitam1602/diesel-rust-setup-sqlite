use crate::{db::establish_connection, models::note::Note, note::NewNote, schema::notes::dsl};
use diesel::prelude::*;

pub fn list_notes() -> Vec<Note> {
    let connection = &mut establish_connection();

    dsl::notes
        .order_by(dsl::title)
        .load::<Note>(connection)
        .expect("Error loading notes")
}

pub fn get_note(id: i32) -> Option<Note> {
    let connection = &mut establish_connection();

    dsl::notes
        .filter(dsl::id.eq(id))
        .first::<Note>(connection)
        .ok()
}

pub fn store_note(note: &NewNote) {
    let connection = &mut establish_connection();

    diesel::insert_into(dsl::notes)
        .values(note)
        .execute(connection)
        .expect("Error saving new note");
}

pub fn update_note(
    id: i32,
    title: String,
    text: String,
    status_id: Option<i32>,
    notebook_id: Option<i32>,
) {
    let connection = &mut establish_connection();

    diesel::update(dsl::notes.filter(dsl::id.eq(id)))
        .set((
            dsl::title.eq(title),
            dsl::text.eq(text),
            dsl::status_id.eq(status_id),
            dsl::notebook_id.eq(notebook_id),
        ))
        .execute(connection)
        .expect("Error updating note");
}
