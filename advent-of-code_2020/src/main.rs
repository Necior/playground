use std::convert::TryInto;
use std::io::{self, BufRead};

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
        let letter = v[1].chars().nth(0).unwrap(); // e.g. 'a'
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

fn main() {
    day02();
}
