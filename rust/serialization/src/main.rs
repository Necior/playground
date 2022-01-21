use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

static DB_FILE: &str = "./person.db";

#[derive(Debug, Deserialize, Serialize)]
enum Exercise {
    Running,
    Swimming,
}

impl Exercise {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "running" => Some(Self::Running),
            "swimming" => Some(Self::Swimming),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    latest_workout: Vec<Exercise>,
}

impl Person {
    fn append_exercise(&mut self, e: Exercise) {
        self.latest_workout.push(e);
    }
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
        Print,
        Add(Exercise),
        Exit,
        Invalid,
        // TODO: add a `Help` command
    }

    impl Command {
        fn from_str(s: &str) -> Self {
            let mut tokens = s.split_ascii_whitespace();
            if let Some(s) = tokens.next() {
                match s {
                    "save" => Save,
                    "load" => Load,
                    "print" => Print,
                    "add" => {
                        if let Some(e) = Exercise::from_str(tokens.next().unwrap_or("")) {
                            Add(e)
                        } else {
                            Invalid
                        }
                    }
                    "exit" => Exit,
                    _ => Invalid,
                }
            } else {
                Invalid
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
                    Command::Print => match person {
                        Some(ref p) => println!("{:?}", p),
                        None => println!("Please load data before trying to print it."),
                    },
                    Command::Add(e) => match person {
                        Some(ref mut p) => p.append_exercise(e),
                        None => println!("Nope. Load data, dude."),
                    },
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
