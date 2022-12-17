use crate::file_reader;
use std::cmp;
use std::collections::HashSet;

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

fn process_move(
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

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_9/input.txt");

    let mut head = Coordination::new();
    let mut tail = Coordination::new();
    let mut visited: HashSet<Coordination> = HashSet::new();

    visited.insert(tail);

    for line in input.lines() {
        let clean_line = remove_whitespace(line);
        let mut chars = clean_line.chars();

        let direction = Direction::char_to_direction(chars.next().unwrap());
        let distance = chars.collect::<String>().parse::<i32>().unwrap();

        process_move(direction, distance, &mut head, &mut tail, &mut visited);
    }

    println!("test: {}", visited.len());
}
