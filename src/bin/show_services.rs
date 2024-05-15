use self::models::Service;
use diesel::prelude::*;
use rata::*;

fn main() {
    use self::schema::services::dsl::*;

    let connection = &mut establish_connection();
    let results = services
        .filter(active.eq(true))
        .limit(5)
        .select(Service::as_select())
        .load(connection)
        .expect("Error loading services");

    println!("Displaying {} services", results.len());
    for service in results {
        println!(
            "{} {} {} {}",
            service.name, service.back_path, service.front_path, service.active
        );
    }
}
