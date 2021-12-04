use std::fs;
const REPORT_BIN_LEN: usize = 12;

#[derive(Debug)]
struct Report<'a> {
    binary_report: &'a Vec<usize>,
    num_ones: [usize; REPORT_BIN_LEN],
    len: usize,
    gamma: usize,
    epsilon: usize,
    pwr_consump: usize,
    oxy_gen_rating: usize,
    co2_scrub_rating: usize,
    life_supp_rating: usize,
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
            oxy_gen_rating: 0,
            co2_scrub_rating: 0,
            life_supp_rating: 0,
        }
    }

    fn calc_epsilon(&mut self) {
        self.epsilon = self.gamma ^ 0xFFF;
    }

    fn calc_pwr_consump(&mut self) {
        self.pwr_consump = self.gamma * self.epsilon;
    }

    fn calc_life_supp(&mut self) {
        self.life_supp_rating = self.oxy_gen_rating * self.co2_scrub_rating;
    }
}

fn find_mcb(num_list: &Vec<usize>, bit: usize) -> usize {
    let mut num_of_ones: usize = 0;

    for num in num_list {
        num_of_ones += num >> (bit - 1) & 0x1;
    }

    let half_len = match num_list.len() / 2 {
        0 => 1.0,
        _ => num_list.len() as f64 / 2.0,
    };

    num_of_ones / half_len.round() as usize
}

fn calc_oxy_rating(nums: &Vec<usize>, bit: usize) -> usize {
    let mcb = find_mcb(nums, bit);
    let filtered_nums: Vec<usize> = nums
        .iter()
        .filter(|&num| (num >> (bit - 1) & 0x1) == mcb)
        .cloned()
        .collect();
    if bit == 1 || filtered_nums.len() == 1 {
        return filtered_nums[0];
    } else {
        calc_oxy_rating(&filtered_nums, bit - 1)
    }
}

fn calc_co2_rating(nums: &Vec<usize>, bit: usize) -> usize {
    let lcb = find_mcb(nums, bit) ^ 0x1;
    let filtered_nums: Vec<usize> = nums
        .iter()
        .filter(|&num| (num >> (bit - 1) & 0x1) == lcb)
        .cloned()
        .collect();
    if bit == 1 || filtered_nums.len() == 1 {
        return filtered_nums[0];
    } else {
        calc_co2_rating(&filtered_nums, bit - 1)
    }
}

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
    report.oxy_gen_rating = calc_oxy_rating(report.binary_report, REPORT_BIN_LEN);
    report.co2_scrub_rating = calc_co2_rating(report.binary_report, REPORT_BIN_LEN);
    report.calc_life_supp();

    println!("-------DAY 3-------");
    println!("Gamma: {}", report.gamma);
    println!("Epsilon: {}", report.epsilon);
    println!("Power Consumption: {}", report.pwr_consump);
    println!("Oxygen Generator Rating: {}", report.oxy_gen_rating);
    println!("CO2 Generator Rating: {}", report.co2_scrub_rating);
    println!("Life Support Rating: {}", report.life_supp_rating);
}
