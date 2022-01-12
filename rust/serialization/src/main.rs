use serde::Serialize;

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    favorite_numbers: Vec<i32>,
}

fn main() {
    let person = Person {
        name: String::from("Necior"),
        favorite_numbers: [21, 37].into(),
    };
    println!("# Debug print");
    println!("  {:?}", person);
    println!("# JSON-serialized");
    println!("  {}", serde_json::to_string(&person).unwrap());
}
