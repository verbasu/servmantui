use rata::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut back_path = String::new();
    let mut front_path = String::new();

    println!("Service name:");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();
    println!("Service back path:");
    stdin().read_line(&mut back_path).unwrap();
    let back_path = back_path.trim_end();
    println!("Service front path:");
    stdin().read_line(&mut front_path).unwrap();
    let front_path = front_path.trim_end();
    let service = create_service(connection, name, back_path, front_path);
    println!("\nSaved service with id {}", service.id);
}
