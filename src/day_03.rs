use core::num;
use std::fs;
use std::mem::size_of;
const REPORT_BIN_LEN: usize = 12;

#[derive(Debug)]
struct Report<'a> {
    binary_report: &'a Vec<usize>,
    num_ones: [usize; REPORT_BIN_LEN],
    len: usize,
    gamma: usize,
    epsilon: usize,
    pwr_consump: usize,
}
impl Report<'_> {
    fn new(len: usize, binary_report: &Vec<usize>) -> Report {
        Report {
            binary_report,
            num_ones: [0; REPORT_BIN_LEN],
            len,
            gamma: 0,
            epsilon: 0,
            pwr_consump: 0,
        }
    }

    fn calc_epsilon(&mut self) {
        self.epsilon = self.gamma ^ 0xFFF;
    }

    fn calc_pwr_consump(&mut self) {
        self.pwr_consump = self.gamma * self.epsilon;
    }
}

fn find_mcb(num_list: &Vec<usize>, bit: usize) -> usize {
    let mut num_of_ones: usize = 0;

    for num in num_list {
        num_of_ones += num >> (bit - 1) & 0x1;
    }
    return num_of_ones / (num_list.len() / 2);
}

// fn calc_oxy_rating(nums: &Vec<usize>) -> usize {
//     if nums.len() == 1 {
//         return nums[0];
//     }
// }

pub fn execute() {
    let input = fs::read_to_string("input/input_03.txt").expect("Unable to open file");
    let binary_nums: Vec<usize> = input
        .lines()
        .map(|n| usize::from_str_radix(n, 2).unwrap())
        .collect();

    let mut report = Report::new(binary_nums.len(), &binary_nums);

    for i in 0..REPORT_BIN_LEN {
        report.num_ones[i] = find_mcb(report.binary_report, REPORT_BIN_LEN - i);
    }

    report.gamma = report.num_ones.iter().fold(0, |acc, &bit| (acc << 1) ^ bit);

    report.calc_epsilon();
    report.calc_pwr_consump();

    println!("Gamma: {}", report.gamma);
    println!("Epsilon: {}", report.epsilon);
    println!("Power Consumption: {}", report.pwr_consump);
}
