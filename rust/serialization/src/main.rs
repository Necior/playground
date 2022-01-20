use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use Handedness::*;

static DB_FILE: &str = "./person.db";

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

fn save_to_file(person: &Person) -> std::io::Result<()> {
    let mut f = File::create(DB_FILE)?;
    let serialized = serde_json::to_string_pretty(person)?;
    f.write_all(serialized.as_bytes())?;
    Ok(())
}

fn load_from_file() -> Result<Person, std::io::Error> {
    let mut f = File::open(DB_FILE)?;
    let mut json = String::new();
    f.read_to_string(&mut json)?;

    Ok(serde_json::from_str(&json).unwrap())
}

fn main() {
    {
        let p = Person {
            name: String::from("Adrian"),
            favorite_numbers: vec![],
            handedness: Right,
        };
        match save_to_file(&p) {
            Ok(_) => println!("Serialized into a file"),
            Err(e) => {
                println!("Serialization failed: {}", e);
                std::process::exit(1);
            }
        };
    }
    match load_from_file() {
        Ok(p) => println!("Deserialization succeeded:\n  {:?}", p),
        Err(e) => {
            println!("Deserialization failed: {}", e);
            std::process::exit(1);
        }
    };

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
        println!(
            "  {}",
            serde_json::to_string(&person).unwrap_or_else(|_| String::from("failed to serialize"))
        );
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
