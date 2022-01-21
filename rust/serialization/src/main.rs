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
    use Command::*;

    #[derive(Debug)]
    enum Command {
        Save,
        Load,
        Exit,
        Invalid,
        // TODO: add a `Help` command
    }

    impl Command {
        fn from_str(s: &str) -> Self {
            match s {
                "save" => Save,
                "load" => Load,
                "exit" => Exit,
                _ => Invalid,
            }
        }
    }

    let mut person: Option<Person> = None;

    loop {
        let mut input = String::with_capacity(8);
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let raw_cmd = input.trim(); // TODO: to lower
                let cmd = Command::from_str(raw_cmd);

                match cmd {
                    Command::Save => {
                        match person {
                            Some(ref p) => {
                                match save_to_file(p) {
                                    Ok(_) => println!("Serialized into a file"),
                                    Err(e) => {
                                        println!("Serialization failed: {}", e);
                                        std::process::exit(1);
                                    }
                                };
                            }
                            None => {
                                println!("Please load data first.");
                            }
                        };
                    }
                    Command::Load => {
                        match load_from_file() {
                            Ok(p) => {
                                person = Some(p);
                            }
                            Err(e) => {
                                println!("Deserialization failed: {}", e);
                                std::process::exit(1);
                            }
                        };
                    }
                    Command::Exit => {
                        std::process::exit(0);
                    }
                    Command::Invalid => {
                        println!("Invalid command");
                    }
                };
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        };
    }
}
