use std::io::{self, BufRead};

fn day01() {
    let nums: Vec<i32> = io::stdin().lock().lines()
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

fn main() {
    day01();
}
