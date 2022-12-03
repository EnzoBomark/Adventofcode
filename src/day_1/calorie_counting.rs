use crate::file_reader;

fn print_largest_sum_of_calories(elfs_calories: &Vec<Vec<i32>>) {
    let max_total_calories: i32 = elfs_calories
        .iter()
        .map(|val| val.iter().sum())
        .max()
        .unwrap();

    println!("Day 01: Part 1: {}", max_total_calories);
}

fn print_combined_value_of_three_largest_sums(elfs_calories: &Vec<Vec<i32>>) {
    let mut sums = elfs_calories
        .iter()
        .map(|val| val.iter().sum())
        .collect::<Vec<i32>>();

    sums.sort();
    sums.reverse();

    println!("Day 01: Part 2: {}", (sums[0] + sums[1] + sums[2]));
}

fn get_vec_of_calories(s: &str) -> Vec<i32> {
    let calories_str: Vec<&str> = s.split("\n").collect();

    calories_str
        .iter()
        .map(|&val| val.parse::<i32>().unwrap())
        .collect()
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_1/input.txt");

    let elfs_calories_str: Vec<&str> = input.split("\n\n").collect();

    let elfs_calories: Vec<Vec<i32>> = elfs_calories_str
        .iter()
        .map(|&val| get_vec_of_calories(val))
        .collect();

    print_largest_sum_of_calories(&elfs_calories);

    print_combined_value_of_three_largest_sums(&elfs_calories);
}
