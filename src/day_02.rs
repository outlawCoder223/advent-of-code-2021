use std::fs;
use std::io;

pub fn execute() -> Result<(), io::Error> {
    let input = fs::read_to_string("input/input_02.txt").expect("Unable to open file");
    let mut input_iter = input.split_whitespace().into_iter();

    let mut y: i32 = 0;
    let mut x: i32 = 0;

    while let Some(direction) = input_iter.next() {
        let units = input_iter.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "up" => y -= units,
            "down" => y += units,
            "forward" => x += units,
            _ => continue,
        }
    }

    println!("Day 2 Answer: {}", x * y);

    Ok(())
}
