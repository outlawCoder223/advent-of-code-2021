use std::fs;

pub fn execute() {
    let input = fs::read_to_string("input/input_01.txt").expect("Unable to read file");
    let input_vec: Vec<i32> = input
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let mut prev: i32 = i32::MAX;
    let mut prev_sum: i32 = i32::MAX;
    let mut sum: i32 = 0;
    let mut count: i32 = 0;
    let mut sum_count: i32 = 0;

    for (i, num) in input_vec.iter().enumerate() {
        if i + 2 < input_vec.len() {
            sum = input_vec[i] + input_vec[i + 1] + input_vec[i + 2];
        }

        if *num > prev {
            count += 1;
        }

        if sum > prev_sum {
            sum_count += 1;
        }
        prev = *num;
        prev_sum = sum;
    }

    println!("-------DAY 1-------");
    println!("Count: {}", count);
    println!("Sum Count: {}", sum_count);
}
