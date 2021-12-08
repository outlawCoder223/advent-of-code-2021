use std::collections::VecDeque; // futile attempt at making the vector work for 256 days
use std::fs;

#[derive(Debug)]
struct Fish {
    internal_timer: usize,
}
impl Fish {
    fn new(internal_timer: usize) -> Fish {
        Fish { internal_timer }
    }

    fn spawn() -> Fish {
        Fish { internal_timer: 8 }
    }

    fn count_day(&mut self) -> Option<Fish> {
        if self.internal_timer <= 0 {
            self.internal_timer = 6;
            return Some(Fish::spawn());
        }
        self.internal_timer -= 1;
        None
    }
}

fn part_1() {
    let input = fs::read_to_string("input/input_06.txt").expect("Unable to open file");

    let mut school: VecDeque<Fish> = input
        .split(",")
        .map(|num| {
            let internal_timer = num.parse::<usize>().unwrap();
            Fish::new(internal_timer)
        })
        .collect();

    let days = 80;

    for _ in 0..days {
        for i in 0..school.len() {
            match school[i].count_day() {
                Some(f) => school.push_back(f),
                None => {}
            }
        }
    }

    println!("Total after 80 days: {}", school.len());
}

fn count_day(school: &mut Vec<usize>) {
    // Position 0 is how many fish are ready to spawn
    let new_fish = school[0];
    // Move each fish to the next day
    for i in 0..school.len() - 1 {
        school[i] = school[i + 1];
    }
    // Add the fish that spawned back to day 6
    school[6] += new_fish;
    // Add the new fish to day 8 since they need 2 extra days to become fertile
    school[8] = new_fish;
}

// Count how many fish in each day of their internal cycle instead of keeping large data structure
fn part_2(days: usize) {
    let input = fs::read_to_string("input/input_06.txt").expect("Unable to open file");

    // The school tracks how many fish in each position (9 positions: 0-8)
    let mut school: Vec<usize> = vec![0; 9];

    // Populate the school with the initial data
    input
        .split(",")
        .for_each(|n| school[n.parse::<usize>().unwrap()] += 1);

    // Loop for X amount of days
    for _ in 0..days {
        count_day(&mut school);
    }

    // Sum the number of fish in each position
    println!(
        "Total after {} days: {}",
        days,
        school.iter().sum::<usize>()
    );
}

pub fn execute() {
    println!("--------DAY 6-------");
    part_1();
    part_2(256);
}
