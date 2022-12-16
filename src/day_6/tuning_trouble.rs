use crate::file_reader;
use std::collections::HashSet;

fn print_four_char_packet_marker(input: &String) {
    let size = 4;

    let marker = input
        .as_bytes()
        .windows(size)
        .enumerate()
        .find_map(|(index, set)| map_unique_hash_set(index, set, size))
        .unwrap();

    println!("Day 06: Part 1: {}", marker);
}

fn print_fourteen_char_packet_marker(input: &String) {
    let size = 14;

    let marker = input
        .as_bytes()
        .windows(size)
        .enumerate()
        .find_map(|(index, set)| map_unique_hash_set(index, set, size))
        .unwrap();

    println!("Day 06: Part 1: {}", marker);
}

fn map_unique_hash_set(index: usize, set: &[u8], size: usize) -> Option<usize> {
    let unique_set = set.iter().copied().collect::<HashSet<u8>>();

    if unique_set.len() == size {
        Some(index + size)
    } else {
        None
    }
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_6/input.txt");

    print_four_char_packet_marker(&input);

    print_fourteen_char_packet_marker(&input);
}
