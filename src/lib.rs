pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::models::{NewService, Service};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_service(
    conn: &mut PgConnection,
    name: &str,
    back_path: &str,
    front_path: &str,
) -> Service {
    use crate::schema::services;

    let new_service = NewService {
        name,
        back_path,
        front_path,
    };

    diesel::insert_into(services::table)
        .values(&new_service)
        .returning(Service::as_returning())
        .get_result(conn)
        .expect("Error saving new service")
}
