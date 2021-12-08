use std::fs;

fn count_1_4_7_8(input: &str) -> usize {
    let digits: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut count = 0;

    for digit in digits {
        match digit.chars().count() {
            2 | 3 | 4 | 7 => count += 1,
            _ => {}
        }
    }

    count
}

fn decode(input: &str) -> usize {
    let digits: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let mut output = String::new();
    for digit in digits {
        match digit.chars().count() {
            2 => output.push('1'),
            3 => output.push('7'),
            4 => output.push('4'),
            7 => output.push('8'),
            _ => output.push('5'),
        }
    }

    output.parse::<usize>().unwrap()
}
pub fn execute() {
    println!("--------DAY 8--------");
    let input = fs::read_to_string("input/test.txt").expect("Couldn't open the file");

    let lines: Vec<Vec<&str>> = input.lines().map(|l| l.split(" | ").collect()).collect();

    let mut count = 0;
    for line in lines.into_iter() {
        count += count_1_4_7_8(line[1]);
        println!("{}", decode(line[1]));
    }

    // println!("{}", count);
}
