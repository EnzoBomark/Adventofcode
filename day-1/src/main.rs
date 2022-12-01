use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut sums: Vec<i32> = Vec::new();
    let mut numbers: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if line == "" {
            sums.push(numbers.iter().sum());
            numbers = Vec::new();
            continue;
        }

        let number: i32 = line.parse().unwrap();
        numbers.push(number);

        if line == contents.lines().last().unwrap() {
            sums.push(numbers.iter().sum());
        }
    }

    fn max_tri_sum(arr: &[i32]) -> i32 {
        let mut arr = Vec::from(arr);
        arr.sort();
        arr.reverse();
        arr[0] + arr[1] + arr[2]
    }

    let biggest_sum = sums.iter().max().unwrap();

    let three_largest_sums = max_tri_sum(&sums);

    println!("Biggest sum: {}", biggest_sum);
    println!("Three biggest sums: {:?}", three_largest_sums);
}
