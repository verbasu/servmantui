use crate::schema::services;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Service {
    pub id: i32,
    pub name: String,
    pub icon_url: Option<String>,
    pub back_path: String,
    pub front_path: String,
    pub back_env: Option<String>,
    pub front_env: Option<String>,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = services)]
pub struct NewService<'a> {
    pub name: &'a str,
    pub front_path: &'a str,
    pub back_path: &'a str,
}
