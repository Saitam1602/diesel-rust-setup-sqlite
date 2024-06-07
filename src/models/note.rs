use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub status_id: Option<i32>,
    pub notebook_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::notes)]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub status_id: Option<i32>,
    pub notebook_id: Option<i32>,
}
