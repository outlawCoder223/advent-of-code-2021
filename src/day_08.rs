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

#[derive(Debug)]
struct DisplayDigit {
    unique_1: Vec<char>,
    unique_4: Vec<char>,
    digits: [String; 10],
}

impl DisplayDigit {
    fn new() -> DisplayDigit {
        DisplayDigit {
            unique_1: Vec::new(),
            unique_4: Vec::new(),
            digits: [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ],
        }
    }

    fn decode(&mut self, input: &str) {
        // break input into 10 strings and sort alphabetically
        let mut digits: Vec<Vec<char>> = input
            .split_whitespace()
            .map(|digit| {
                let mut sorted_digit: Vec<char> = digit.chars().collect();
                sorted_digit.sort_by(|a, b| a.cmp(b));
                return sorted_digit;
            })
            .collect();
        digits.sort_by(|a, b| a.len().cmp(&b.len()));

        for digit in digits.iter() {
            match digit.len() {
                2 => {
                    // make sure to grab the 2 segs of one for later use
                    for c in digit {
                        self.digits[1].push(*c);
                        self.unique_1.push(*c);
                    }
                }
                3 => {
                    // 7 is only num with 3 segs
                    for c in digit {
                        self.digits[7].push(*c);
                    }
                }
                4 => {
                    // grab the 2 segs that are not the segs from 1
                    for c in digit {
                        self.digits[4].push(*c);
                        if !self.unique_1.contains(&c) {
                            self.unique_4.push(*c);
                        }
                    }
                }
                5 => {
                    // 3 has both segs from 1
                    if digit.contains(&self.unique_1[1]) && digit.contains(&self.unique_1[0]) {
                        for c in digit {
                            self.digits[3].push(*c);
                        }
                    // 5 has both segs of 4
                    } else if digit.contains(&self.unique_4[1]) && digit.contains(&self.unique_4[0])
                    {
                        for c in digit {
                            self.digits[5].push(*c);
                        }
                    // 2 is what is left over
                    } else {
                        for c in digit {
                            self.digits[2].push(*c);
                        }
                    }
                }
                6 => {
                    // 6 is missing a seg of 1
                    if !digit.contains(&self.unique_1[1]) || !digit.contains(&self.unique_1[0]) {
                        for c in digit {
                            self.digits[6].push(*c);
                        }
                    // 0 is missing a seg of 4
                    } else if !digit.contains(&self.unique_4[1])
                        || !digit.contains(&self.unique_4[0])
                    {
                        for c in digit {
                            self.digits[0].push(*c);
                        }
                    // 9 is what is left over
                    } else {
                        for c in digit {
                            self.digits[9].push(*c);
                        }
                    }
                }
                7 => {
                    // 8 is only num with 7 segs
                    for c in digit {
                        self.digits[8].push(*c);
                    }
                }
                _ => println!("This makes no sense..."),
            }
        }
    }

    fn get_digits(&self, seg: &str) -> usize {
        let segs: Vec<String> = seg
            .split_whitespace()
            .map(|digit| {
                let mut sorted_digit: Vec<char> = digit.chars().collect();
                sorted_digit.sort_by(|a, b| a.cmp(b));
                return String::from_iter(sorted_digit);
            })
            .collect();

        let digit1 = self.digits.iter().position(|d| d.eq(&segs[0])).unwrap();
        let digit2 = self.digits.iter().position(|d| d.eq(&segs[1])).unwrap();
        let digit3 = self.digits.iter().position(|d| d.eq(&segs[2])).unwrap();
        let digit4 = self.digits.iter().position(|d| d.eq(&segs[3])).unwrap();

        digit1 * 1000 + digit2 * 100 + digit3 * 10 + digit4
    }
}

pub fn execute() {
    println!("--------DAY 8--------");
    let input = fs::read_to_string("input/input_08.txt").expect("Couldn't open the file");

    let lines: Vec<Vec<&str>> = input.lines().map(|l| l.split(" | ").collect()).collect();

    let mut count1 = 0;
    let mut count2 = 0;
    for line in lines.into_iter() {
        let mut display_digit = DisplayDigit::new();
        count1 += count_1_4_7_8(line[1]);
        display_digit.decode(line[0]);
        count2 += display_digit.get_digits(line[1]);
    }

    println!("Part 1 answer: {}", count1);
    println!("Part 2 answer: {}", count2);
}
