use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notebooks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Notebook {
    pub id: i32,
    pub title: String,
    pub status_id: Option<i32>,
    pub notebook_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::notebooks)]
pub struct NewNotebook<'a> {
    pub title: &'a str,
    pub status_id: Option<i32>,
    pub notebook_id: Option<i32>,
}
