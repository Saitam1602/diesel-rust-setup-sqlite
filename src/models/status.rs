use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::statuses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Status {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::statuses)]
pub struct NewStatus<'a> {
    pub name: &'a str,
    pub color: &'a str,
}
