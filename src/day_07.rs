use std::fs;

pub fn execute() {
    let input = fs::read_to_string("input/input_07.txt").expect("Unable to open file");
    let nums: Vec<i32> = input
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mean = nums.iter().sum::<i32>() / nums.len() as i32;

    let mut min_fuel: i32 = i32::MAX;
    let mut current_fuel: i32 = 0;

    for i in 0..mean {
        for num in nums.iter() {
            current_fuel += i32::abs(num - i);
        }
        if current_fuel < min_fuel {
            min_fuel = current_fuel;
        }
        current_fuel = 0;
    }

    println!("Max Fuel: {}", min_fuel);
}
