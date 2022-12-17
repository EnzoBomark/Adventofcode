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

fn compute_tail_move(from: Coordination, to: Coordination) -> Coordination {
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

fn compute_rope(
    direction: Direction,
    distance: i32,
    head: &mut Coordination,
    tail: &mut Coordination,
    visited: &mut HashSet<Coordination>,
) {
    for _ in 1..=distance {
        match direction {
            Direction::Up => head.y += 1,
            Direction::Right => head.x += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
        }

        let delta_x = (tail.x - head.x).abs();
        let delta_y = (tail.y - head.y).abs();

        if cmp::max(delta_x, delta_y) > 1 {
            let delta = compute_tail_move(*tail, *head);
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
        match direction {
            Direction::Up => knots[0].y += 1,
            Direction::Right => knots[0].x += 1,
            Direction::Down => knots[0].y -= 1,
            Direction::Left => knots[0].x -= 1,
        }

        for n in 1..=TAIL_LENGTH {
            let sn = n as usize;

            let delta_x = (knots[sn].x - knots[sn - 1].x).abs();
            let delta_y = (knots[sn].y - knots[sn - 1].y).abs();

            if cmp::max(delta_x, delta_y) > 1 {
                let delta = compute_tail_move(knots[sn], knots[sn - 1]);
                knots[sn].x += delta.x;
                knots[sn].y += delta.y;
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
