use std::collections::VecDeque;
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

pub fn execute() {
    println!("--------DAY 6-------");
    part_1();
}
