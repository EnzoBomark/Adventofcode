use crate::file_reader;

struct Instructions {
    quantity: usize,
    from: usize,
    to: usize,
}

fn get_stacks_list(table: Vec<&str>) -> Vec<Vec<char>> {
    let stacks_table: Vec<Vec<char>> = table
        .iter()
        .map(|&val| val.chars().skip(1).step_by(4).collect())
        .collect();

    let mut stack_lists: Vec<Vec<char>> = Vec::new();

    for i in 0..9 {
        let mut stack_list: Vec<char> = Vec::new();

        for j in 0..8 {
            let has_crate = stacks_table[j][i].is_alphabetic();

            if has_crate {
                stack_list.push(stacks_table[j][i]);
            }
        }

        stack_list.reverse();

        stack_lists.push(stack_list);
    }

    stack_lists
}

fn get_crane_moves(crane_moves: Vec<&str>) -> Vec<Instructions> {
    crane_moves
        .iter()
        .map(|&val| {
            let instructions = val
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .skip(1)
                .step_by(2)
                .map(|&val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let quantity = instructions[0];
            let from = instructions[1] - 1;
            let to = instructions[2] - 1;

            Instructions { quantity, from, to }
        })
        .collect()
}

fn get_rearrangement_combined_top_ids(rearrangements: Vec<Vec<char>>) -> String {
    let mut final_rearrangement_combined_top_ids: String = String::new();

    for i in 0..9 {
        let stack = rearrangements[i].clone();
        let last_crate = stack.last().unwrap().to_string();
        final_rearrangement_combined_top_ids.push_str(&last_crate);
    }

    final_rearrangement_combined_top_ids
}

fn print_rearrangement_for_crate_mover_9000(
    stacks_list: &Vec<Vec<char>>,
    crane_moves: &Vec<Instructions>,
) {
    let mut rearrangements = stacks_list.clone();

    for instruction in crane_moves {
        let mut from_stack = rearrangements[instruction.from].clone();
        let mut to_stack = rearrangements[instruction.to].clone();

        for _ in 0..instruction.quantity {
            let crate_to_move = from_stack.pop();

            match crate_to_move {
                Some(value) => to_stack.push(value),
                None => {}
            }
        }

        rearrangements[instruction.from] = from_stack;
        rearrangements[instruction.to] = to_stack;
    }

    let final_rearrangement_combined_top_ids = get_rearrangement_combined_top_ids(rearrangements);

    println!("Day 05: Part 1: {}", final_rearrangement_combined_top_ids);
}

fn print_rearrangement_for_crate_mover_9001(
    stacks_list: &Vec<Vec<char>>,
    crane_moves: &Vec<Instructions>,
) {
    let mut rearrangements = stacks_list.clone();

    for instruction in crane_moves {
        let mut from_stack = rearrangements[instruction.from].clone();
        let mut to_stack = rearrangements[instruction.to].clone();

        let mut crates = Vec::new();

        for _ in 0..instruction.quantity {
            let crate_to_move = from_stack.pop();

            match crate_to_move {
                Some(value) => crates.push(value),
                None => {}
            }
        }

        crates.reverse();
        to_stack.append(&mut crates);

        rearrangements[instruction.from] = from_stack;
        rearrangements[instruction.to] = to_stack;
    }

    let final_rearrangement_combined_top_ids = get_rearrangement_combined_top_ids(rearrangements);

    println!("Day 05: Part 2: {}", final_rearrangement_combined_top_ids);
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_5/input.txt");
    let organized_input: Vec<&str> = input.split("\n\n").collect();

    let stacks_str: Vec<&str> = organized_input[0].split("\n").collect();
    let stacks_list = get_stacks_list(stacks_str);

    let crane_moves_str: Vec<&str> = organized_input[1].split("\n").collect();
    let crane_moves = get_crane_moves(crane_moves_str);

    print_rearrangement_for_crate_mover_9000(&stacks_list, &crane_moves);

    print_rearrangement_for_crate_mover_9001(&stacks_list, &crane_moves);
}
