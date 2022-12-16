use crate::file_reader;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Node<'a> {
    is_file: bool,
    size: Option<u64>,
    parent: Option<Rc<RefCell<Node<'a>>>>,
    sub_dirs: HashMap<&'a str, Rc<RefCell<Node<'a>>>>,
}

impl<'a> Node<'a> {
    fn size(&self) -> u64 {
        if self.is_file {
            self.size.unwrap()
        } else {
            self.sub_dirs.values().map(|n| n.borrow().size()).sum()
        }
    }

    fn size_at_most(&self, limit: u64) -> u64 {
        if self.is_file {
            return 0;
        }

        let size = self.size();

        let ans = if size <= limit { size } else { 0 }
            + self
                .sub_dirs
                .values()
                .map(|n| n.borrow().size_at_most(limit))
                .sum::<u64>();

        ans
    }

    fn free_up_space(&self, to_free: u64) -> Option<u64> {
        if self.is_file {
            return None;
        }

        let sub_dir_size = self
            .sub_dirs
            .values()
            .filter_map(|n| n.borrow().free_up_space(to_free))
            .min();

        if sub_dir_size.is_some() {
            return sub_dir_size;
        }

        let size = self.size();

        if size >= to_free {
            return Some(size);
        }

        None
    }
}

fn parse_file_system(input: &str) -> Node {
    let index_node = Node {
        size: None,
        is_file: false,
        parent: None,
        sub_dirs: Default::default(),
    };

    let root = Rc::new(RefCell::new(index_node));

    let mut current_node = Rc::clone(&root);

    for line in input.trim().lines() {
        let line_ascii = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let ascii_slice = line_ascii.as_slice();

        match ascii_slice {
            ["$", "cd", path] => match *path {
                "/" => {
                    let root = Rc::clone(&root);
                    current_node = root
                }

                ".." => {
                    let parent = Rc::clone(current_node.borrow().parent.as_ref().unwrap());
                    current_node = parent
                }

                path => {
                    let sub_dir = Rc::clone(current_node.borrow().sub_dirs.get(path).unwrap());
                    current_node = sub_dir
                }
            },
            ["dir", dir] => {
                let dir_node = Node {
                    size: None,
                    is_file: false,
                    parent: Some(Rc::clone(&current_node)),
                    sub_dirs: Default::default(),
                };

                current_node
                    .borrow_mut()
                    .sub_dirs
                    .insert(dir, Rc::new(RefCell::new(dir_node)));
            }
            ["$", "ls"] => (),
            [size, file] => {
                let file_node = Node {
                    size: Some(size.parse().unwrap()),
                    is_file: true,
                    parent: Some(Rc::clone(&current_node)),
                    sub_dirs: Default::default(),
                };

                current_node
                    .borrow_mut()
                    .sub_dirs
                    .insert(file, Rc::new(RefCell::new(file_node)));
            }
            _ => unreachable!(),
        }
    }

    return root.borrow().clone();
}

fn print_sum_of_nodes_with_size_smaller_than_limit(input: &str, limit: u64) {
    let root = parse_file_system(input);
    let sum = root.size_at_most(limit).to_string();

    println!("Day 07: Part 1: {}", sum);
}

fn print_sum_of_deleted_nodes_to_reach_desired_space(input: &str, total: u64, required: u64) {
    let root = parse_file_system(input);
    let curr = total - root.size();
    let to_free = required - curr;
    let sum = root.free_up_space(to_free).unwrap().to_string();

    println!("Day 07: Part 2: {}", sum);
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_7/input.txt");

    print_sum_of_nodes_with_size_smaller_than_limit(&input, 100000);

    print_sum_of_deleted_nodes_to_reach_desired_space(&input, 70000000, 30000000);
}
