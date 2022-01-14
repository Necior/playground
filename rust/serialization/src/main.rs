use serde::{Deserialize, Serialize};
use Handedness::*;

#[derive(Debug, Deserialize, Serialize)]
enum Handedness {
    Right,
    Left,
    Other(String),
}

impl std::fmt::Display for Handedness {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Right => "right",
            Left => "left",
            Other(o) => o,
        };
        write!(fmt, "{}", text)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    favorite_numbers: Vec<i32>,
    handedness: Handedness,
}

fn main() {
    let hs = vec![Right, Left, Other(String::from("both"))];
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

    let json =
        r#"{"name": "Necior", "favorite_numbers": [21, 37], "handedness": {"Other": "both"}}"#;
    match serde_json::from_str(json) {
        Ok(Person {
            name, handedness, ..
        }) => println!("{} is {}-handed", name, handedness),
        Err(e) => eprintln!("{}", e),
    };
}
