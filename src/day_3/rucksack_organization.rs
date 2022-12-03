use crate::file_reader;

fn char_to_priority(c: char) -> i32 {
    let ascii_code = c as i32;

    if ascii_code >= 65 && ascii_code <= 90 {
        ascii_code - 65 + 27
    } else {
        ascii_code - 97 + 1
    }
}

fn find_matching_char_priority_for_rucksack(rucksack: &str) -> i32 {
    let (compartment_one, compartment_two) = rucksack.split_at(rucksack.len() / 2);

    let matching_char = compartment_one
        .chars()
        .find(|val| compartment_two.contains(*val))
        .unwrap();

    char_to_priority(matching_char)
}

fn find_matching_identifier_priority_for_rucksack(
    group_index: usize,
    rucksacks: &Vec<&str>,
) -> i32 {
    let rucksack_1 = rucksacks[group_index];
    let rucksack_2 = rucksacks[group_index + 1];
    let rucksack_3 = rucksacks[group_index + 2];

    let matching_char = rucksack_1
        .chars()
        .find(|val| rucksack_2.contains(*val) && rucksack_3.contains(*val))
        .unwrap();

    char_to_priority(matching_char)
}

fn print_rucksack_organization_sum(rucksacks: &Vec<&str>) {
    let sum: i32 = rucksacks
        .iter()
        .map(|val| find_matching_char_priority_for_rucksack(val))
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("Day 03: Part 1: {}", sum);
}

fn print_elf_group_identifier_sums(rucksacks: &Vec<&str>) {
    let sums: i32 = (0..rucksacks.len())
        .step_by(3)
        .map(|group_index| find_matching_identifier_priority_for_rucksack(group_index, rucksacks))
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("Day 03: Part 2: {}", sums);
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_3/input.txt");

    let rucksacks: Vec<&str> = input.split('\n').collect();

    print_rucksack_organization_sum(&rucksacks);

    print_elf_group_identifier_sums(&rucksacks);
}
