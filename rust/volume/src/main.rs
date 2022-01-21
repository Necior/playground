fn generate_values(min: i128, step: i128, max: i128) -> Vec<i128> {
    let mut v = vec![min];
    let mut current = min;
    loop {
        current = current + step;
        if current > max {
            break;
        }
        v.push(current);
    }
    v
}

#[derive(Debug, PartialEq, Eq)]
struct Variant {
    mass: i128,
    sets: i128,
    reps: i128,
}

impl Variant {
    fn volume(&self) -> i128 {
        self.mass * self.sets * self.reps
    }
}

impl std::cmp::Ord for Variant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.volume().cmp(&other.volume())
    }
}

impl PartialOrd for Variant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.volume().partial_cmp(&other.volume())
    }
}

fn main() {
    let available_mass = generate_values(20, 5, 80);
    let available_sets = generate_values(3, 1, 4);
    let available_reps = generate_values(5, 1, 8);

    let mut variants = vec![];

    for mass in &available_mass {
        for sets in &available_sets {
            for reps in &available_reps {
                let variant = Variant {
                    mass: *mass,
                    sets: *sets,
                    reps: *reps,
                };
                variants.push(variant);
            }
        }
    }
    variants.sort();

    println!("{:#?}", variants);
    println!("Total: {}", variants.len());
}
