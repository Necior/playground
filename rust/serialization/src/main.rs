use serde::Serialize;
use Handedness::*;

#[derive(Debug, Serialize)]
enum Handedness {
    Right,
    Left,
    Other(String),
}

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    favorite_numbers: Vec<i32>,
    handedness: Handedness,
}

fn main() {
    let hs: Vec<Handedness> = vec![Right, Left, Other(String::from("both"))];
    for h in hs {
        let person = Person {
            name: String::from("Necior"),
            favorite_numbers: [21, 37].into(),
            handedness: h,
        };
        println!("# Debug print");
        println!("  {:?}", person);
        println!("# JSON-serialized");
        println!("  {}", serde_json::to_string(&person).unwrap());
    }
}
