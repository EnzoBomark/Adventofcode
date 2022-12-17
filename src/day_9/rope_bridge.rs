use crate::file_reader;
use std::cmp;
use std::collections::HashSet;

const TAIL_LENGTH: usize = 9;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn char_to_direction(char: char) -> Direction {
        match char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Move {
    direction: Direction,
    distance: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordination {
    pub x: i32,
    pub y: i32,
}

impl Coordination {
    fn new() -> Coordination {
        Coordination { x: 0, y: 0 }
    }
}

fn get_tail_move(from: Coordination, to: Coordination) -> Coordination {
    let mut delta_x: i32 = to.x - from.x;
    let mut delta_y: i32 = to.y - from.y;

    if (delta_x.abs() <= 2) && (delta_y.abs() <= 2) {
        delta_x = delta_x.clamp(-1, 1);
        delta_y = delta_y.clamp(-1, 1);
    } else if delta_x.abs() == 2 && delta_y == 0 {
        delta_x = delta_x.clamp(-1, 1);
    } else if delta_x == 0 && delta_y.abs() == 2 {
        delta_y = delta_y.clamp(-1, 1);
    }

    Coordination {
        x: delta_x,
        y: delta_y,
    }
}

fn move_rope_node(direction: Direction, node: &mut Coordination) {
    match direction {
        Direction::Up => node.y += 1,
        Direction::Right => node.x += 1,
        Direction::Down => node.y -= 1,
        Direction::Left => node.x -= 1,
    }
}

fn should_move_tail(tail: Coordination, head: Coordination) -> bool {
    let v1 = (tail.x - head.x).abs();
    let v2 = (tail.y - head.y).abs();
    cmp::max(v1, v2) > 1
}

fn compute_rope(
    direction: Direction,
    distance: i32,
    head: &mut Coordination,
    tail: &mut Coordination,
    visited: &mut HashSet<Coordination>,
) {
    for _ in 1..=distance {
        move_rope_node(direction, head);

        if should_move_tail(*tail, *head) {
            let delta = get_tail_move(*tail, *head);
            tail.x += delta.x;
            tail.y += delta.y;
            visited.insert(*tail);
        }
    }
}

fn compute_rope_with_knots(
    direction: Direction,
    distance: i32,
    knots: &mut Vec<Coordination>,
    visited: &mut HashSet<Coordination>,
) {
    for _ in 1..=distance {
        move_rope_node(direction, &mut knots[0]);

        for i in 1..=TAIL_LENGTH {
            if should_move_tail(knots[i], knots[i - 1]) {
                let delta = get_tail_move(knots[i], knots[i - 1]);
                knots[i].x += delta.x;
                knots[i].y += delta.y;
            }
        }

        visited.insert(knots[TAIL_LENGTH]);
    }
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn get_direction_and_distance_from_string(string: &str) -> Move {
    let clean_line = remove_whitespace(string);
    let mut chars = clean_line.chars();

    let direction = Direction::char_to_direction(chars.next().unwrap());
    let distance = chars.collect::<String>().parse::<i32>().unwrap();

    Move {
        direction,
        distance,
    }
}

fn get_short_rope_visited_coordinations_length(input: &String) -> usize {
    let mut head = Coordination::new();
    let mut tail = Coordination::new();
    let mut visited: HashSet<Coordination> = HashSet::new();

    visited.insert(tail);

    for line in input.lines() {
        let input_move = get_direction_and_distance_from_string(line);

        compute_rope(
            input_move.direction,
            input_move.distance,
            &mut head,
            &mut tail,
            &mut visited,
        );
    }

    visited.len()
}

fn get_long_rope_visited_coordinations_length(input: &String) -> usize {
    let mut knots: Vec<Coordination> = vec![];

    for _ in 0..=TAIL_LENGTH {
        knots.push(Coordination::new());
    }

    let mut visited: HashSet<Coordination> = HashSet::new();

    visited.insert(knots[TAIL_LENGTH]);

    for line in input.lines() {
        let input_move = get_direction_and_distance_from_string(line);

        compute_rope_with_knots(
            input_move.direction,
            input_move.distance,
            &mut knots,
            &mut visited,
        );
    }

    visited.len()
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_9/input.txt");

    println!(
        "Day 9, part 1: {:?}",
        get_short_rope_visited_coordinations_length(&input)
    );

    println!(
        "Day 9, part 2: {:?}",
        get_long_rope_visited_coordinations_length(&input)
    );
}
