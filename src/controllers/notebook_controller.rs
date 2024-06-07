use crate::{
    db::establish_connection, models::notebook::Notebook, notebook::NewNotebook,
    schema::notebooks::dsl,
};
use diesel::prelude::*;

pub fn list_notebooks() -> Vec<Notebook> {
    let connection = &mut establish_connection();

    dsl::notebooks
        .order_by(dsl::title)
        .load::<Notebook>(connection)
        .expect("Error loading notebooks")
}

pub fn get_notebook(id: i32) -> Option<Notebook> {
    let connection = &mut establish_connection();

    dsl::notebooks
        .filter(dsl::id.eq(id))
        .first::<Notebook>(connection)
        .ok()
}

pub fn store_notebook(notebook: &NewNotebook) {
    let connection = &mut establish_connection();

    diesel::insert_into(dsl::notebooks)
        .values(notebook)
        .execute(connection)
        .expect("Error saving new notebook");
}

pub fn update_notebook(id: i32, title: String, status_id: Option<i32>, notebook_id: Option<i32>) {
    let connection = &mut establish_connection();

    diesel::update(dsl::notebooks.filter(dsl::id.eq(id)))
        .set((
            dsl::title.eq(title),
            dsl::status_id.eq(status_id),
            dsl::notebook_id.eq(notebook_id),
        ))
        .execute(connection)
        .expect("Error updating notebook");
}
