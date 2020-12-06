use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, BufRead, Read};

#[allow(dead_code)]
fn day01() {
    let nums: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();
    'part1: for n1 in &nums {
        for n2 in &nums {
            if n1 + n2 == 2020 {
                println!("Part 1");
                println!("{}", n1 * n2);
                break 'part1;
            }
        }
    }

    'part2: for n1 in &nums {
        for n2 in &nums {
            for n3 in &nums {
                if n1 + n2 + n3 == 2020 {
                    println!("Part 2");
                    println!("{}", n1 * n2 * n3);
                    break 'part2;
                }
            }
        }
    }
}

#[allow(dead_code)]
fn day02() {
    let mut counter_a = 0;
    let mut counter_b = 0;
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    for line in &lines {
        let v: Vec<_> = line.split(' ').collect();
        let from_to: Vec<u32> = v[0].split('-').map(|s| s.parse().unwrap()).collect();

        // e.g.: "1-3 a: abcde"
        let min = from_to[0]; // e.g. 1
        let max = from_to[1]; // e.g. 3
        let letter = v[1].chars().next().unwrap(); // e.g. 'a'
        let password = v[2]; // e.g. "abcde"

        let count: u32 = password
            .chars()
            .filter(|c| c == &letter)
            .count()
            .try_into()
            .unwrap();

        // Part One
        if min <= count && count <= max {
            counter_a += 1;
        }

        // Part Two
        let pos1 = password.chars().nth((min as usize) - 1).unwrap() == letter;
        let pos2 = password.chars().nth((max as usize) - 1).unwrap() == letter;
        counter_b += match pos1 ^ pos2 {
            false => 0,
            true => 1,
        };
    }
    println!("# of valid passwords (Part One): {}", counter_a);
    println!("# of valid passwords (Part Two): {}", counter_b);
}

#[allow(dead_code)]
fn day03() {
    fn is_tree(line: &str, line_number: usize, right: usize, down: usize) -> bool {
        if (line_number + 1) % down != 0 {
            return false;
        }
        let index = (right * (line_number / down + 1)) % line.len();
        line.chars().nth(index).unwrap() == '#'
    }

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut encountered_trees = [0; 5];

    for (line_no, line) in io::stdin().lock().lines().enumerate() {
        if line_no == 0 {
            continue;
        }
        let line = line.unwrap();
        for i in 0..encountered_trees.len() {
            if is_tree(&line, line_no - 1, slopes[i].0, slopes[i].1) {
                encountered_trees[i] += 1;
            }
        }
    }
    println!("Part One: {}", encountered_trees[1]);
    let total: u128 = encountered_trees.iter().product();
    println!("Part Two: {}", total);
}

#[allow(dead_code)]
fn day04() {
    pub mod validators {
        use std::collections::HashMap;

        pub enum ValidationResult {
            EverythingOk,
            MissingFields,
            InvalidFields,
        }

        fn byr(value: &str) -> bool {
            // four digits; at least 1920 and at most 2002.
            let year: u16 = value.parse().unwrap();
            1920 <= year && year <= 2002
        }

        fn iyr(value: &str) -> bool {
            // four digits; at least 2010 and at most 2020.
            let year: u16 = value.parse().unwrap();
            2010 <= year && year <= 2020
        }

        fn eyr(value: &str) -> bool {
            // four digits; at least 2020 and at most 2030.
            let year: u16 = value.parse().unwrap();
            2020 <= year && year <= 2030
        }

        fn hgt(value: &str) -> bool {
            // a number followed by either cm or in:
            //
            //     If cm, the number must be at least 150 and at most 193.
            //     If in, the number must be at least 59 and at most 76.
            if value.ends_with("cm") && value.len() >= 5 {
                let height: u8 = value[0..3].parse().unwrap();
                150 <= height && height <= 193
            } else if value.ends_with("in") && value.len() >= 4 {
                let height: u8 = value[0..2].parse().unwrap();
                59 <= height && height <= 76
            } else {
                false
            }
        }

        fn hcl(value: &str) -> bool {
            // a # followed by exactly six characters 0-9 or a-f.
            value.starts_with('#')
                && value.len() == 7
                && u32::from_str_radix(&value[1..7], 16).is_ok()
        }

        fn ecl(value: &str) -> bool {
            // exactly one of: amb blu brn gry grn hzl oth.
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
        }

        fn pid(value: &str) -> bool {
            // a nine-digit number, including leading zeroes.
            value.len() == 9 && value.parse::<u32>().is_ok()
        }

        pub fn validate_fields(fields: HashMap<&str, &str>) -> ValidationResult {
            let mandatory_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
            let validators = [byr, iyr, eyr, hgt, hcl, ecl, pid];
            for field in mandatory_fields.iter() {
                if !fields.contains_key(field) {
                    return ValidationResult::MissingFields;
                }
            }
            for (i, validator) in validators.iter().enumerate() {
                if !validator(fields.get(mandatory_fields[i]).unwrap()) {
                    return ValidationResult::InvalidFields;
                }
            }
            ValidationResult::EverythingOk
        }
    }

    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let raw_passports = buffer.split("\n\n");

    let mut passports_with_required_fields = 0;
    let mut passports_with_valid_fields = 0;

    for raw_passport in raw_passports {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        for mut splitter in raw_passport.split_whitespace().map(|f| f.splitn(2, ':')) {
            let field = splitter.next().unwrap();
            let value = splitter.next().unwrap();
            fields.insert(field, value);
        }
        match validators::validate_fields(fields) {
            validators::ValidationResult::EverythingOk => {
                passports_with_valid_fields += 1;
                passports_with_required_fields += 1;
            }
            validators::ValidationResult::MissingFields => {
                // Don't increment any counter
            }
            validators::ValidationResult::InvalidFields => passports_with_required_fields += 1,
        };
    }

    println!("Part One: {}", passports_with_required_fields);
    println!("Part Two: {}", passports_with_valid_fields);
}

fn day05() {
    #[derive(Clone)]
    enum Half {
        Lower,
        Upper,
    }
    struct Line {
        binary_row: Vec<Half>,
        binary_col: Vec<Half>,
    }
    struct Seat {
        row: u16,
        col: u16,
    }

    impl Seat {
        fn get_id(&self) -> u16 {
            self.row * 8 + self.col
        }
    }
    fn bisect(mut min: u16, mut max: u16, halves: Vec<Half>) -> u16 {
        for half in halves {
            let lm = match half {
                Half::Lower => (min, max - (max - min) / 2 - 1),
                Half::Upper => (min + (max - min) / 2 + 1, max),
            };
            min = lm.0;
            max = lm.1;
        }
        assert_eq!(min, max);
        min
    }
    fn parse_line(line: &str) -> Line {
        // e.g.: "FBFBBFFRLR"
        assert_eq!(line.len(), 10);
        let halves: Vec<_> = line
            .chars()
            .map(|c| match c {
                'F' => Half::Lower,
                'L' => Half::Lower,
                'B' => Half::Upper,
                'R' => Half::Upper,
                _ => unreachable!("unexpected input"),
            })
            .collect();
        let (br, bc) = halves.split_at(7);
        Line {
            binary_row: br.into(),
            binary_col: bc.into(),
        }
    }
    fn get_seat(line: &str) -> Seat {
        let parsed = parse_line(line);
        Seat {
            row: bisect(0, 127, parsed.binary_row),
            col: bisect(0, 7, parsed.binary_col),
        }
    }

    let seat_ids: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| get_seat(&line.unwrap()).get_id())
        .collect();

    let min_seat_id = *seat_ids.iter().min().unwrap();
    let max_seat_id = *seat_ids.iter().max().unwrap();

    println!("Part One: {}", max_seat_id);

    for seat_id in min_seat_id..max_seat_id {
        if !seat_ids.contains(&seat_id) {
            println!("Part Two: {}", seat_id);
            break;
        }
    }
}

fn main() {
    day05();
}
