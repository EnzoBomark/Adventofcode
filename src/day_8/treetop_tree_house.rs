use crate::file_reader;

struct Grid {
    trees: Vec<Vec<Tree>>,
}

#[derive(Clone)]
struct Tree {
    height: i8,
    coordinate: Coordinate,
}

#[derive(Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Grid {
    fn new() -> Grid {
        Grid { trees: vec![] }
    }

    fn add_tree(&mut self, tree: Tree) {
        let x = tree.coordinate.x as usize;
        let y = tree.coordinate.y as usize;

        if self.trees.len() <= y {
            self.trees.push(vec![]);
        }

        if self.trees[y].len() <= x {
            self.trees[y].resize(
                x + 1,
                Tree {
                    height: 0,
                    coordinate: Coordinate {
                        x: x as i32,
                        y: y as i32,
                    },
                },
            );
        }

        self.trees[y][x] = tree;
    }

    fn is_visible_from_left(&self, tree: &Tree) -> bool {
        self.trees[tree.coordinate.y as usize]
            .iter()
            .take(tree.coordinate.x as usize)
            .all(|t| t.height < tree.height)
    }

    fn is_visible_from_right(&self, tree: &Tree) -> bool {
        self.trees[tree.coordinate.y as usize]
            .iter()
            .skip(tree.coordinate.x as usize + 1)
            .all(|t| t.height < tree.height)
    }

    fn is_visible_from_top(&self, tree: &Tree) -> bool {
        self.trees
            .iter()
            .take(tree.coordinate.y as usize)
            .all(|row| row[tree.coordinate.x as usize].height < tree.height)
    }

    fn is_visible_from_bottom(&self, tree: &Tree) -> bool {
        self.trees
            .iter()
            .skip(tree.coordinate.y as usize + 1)
            .all(|row| row[tree.coordinate.x as usize].height < tree.height)
    }

    fn is_visible(&self, tree: &Tree) -> bool {
        self.is_visible_from_left(tree)
            || self.is_visible_from_right(tree)
            || self.is_visible_from_top(tree)
            || self.is_visible_from_bottom(tree)
    }

    fn get_visible_trees_count(&self) -> usize {
        self.trees
            .iter()
            .flatten()
            .filter(|tree| self.is_visible(tree))
            .count()
    }

    fn get_left_visible_length(&self, tree: &Tree) -> usize {
        let mut visible_length = 0;

        for x in (0..tree.coordinate.x).rev() {
            let current_tree = &self.trees[tree.coordinate.y as usize][x as usize];

            if current_tree.height < tree.height {
                visible_length += 1;
            } else {
                visible_length += 1;
                break;
            }
        }

        visible_length
    }

    fn get_right_visible_length(&self, tree: &Tree) -> usize {
        let mut visible_length = 0;

        for x in tree.coordinate.x + 1..self.trees[tree.coordinate.y as usize].len() as i32 {
            let current_tree = &self.trees[tree.coordinate.y as usize][x as usize];

            if current_tree.height < tree.height {
                visible_length += 1;
            } else {
                visible_length += 1;
                break;
            }
        }

        visible_length
    }

    fn get_top_visible_length(&self, tree: &Tree) -> usize {
        let mut visible_length = 0;

        for y in (0..tree.coordinate.y).rev() {
            let current_tree = &self.trees[y as usize][tree.coordinate.x as usize];

            if current_tree.height < tree.height {
                visible_length += 1;
            } else {
                visible_length += 1;
                break;
            }
        }

        visible_length
    }

    fn get_bottom_visible_length(&self, tree: &Tree) -> usize {
        let mut visible_length = 0;

        for y in tree.coordinate.y + 1..self.trees.len() as i32 {
            let current_tree = &self.trees[y as usize][tree.coordinate.x as usize];

            if current_tree.height < tree.height {
                visible_length += 1;
            } else {
                visible_length += 1;
                break;
            }
        }

        visible_length
    }

    fn get_visible_length(&self, tree: &Tree) -> usize {
        self.get_left_visible_length(tree)
            * self.get_right_visible_length(tree)
            * self.get_top_visible_length(tree)
            * self.get_bottom_visible_length(tree)
    }

    fn get_max_visible_length(&self) -> i32 {
        self.trees
            .iter()
            .flatten()
            .map(|tree| self.get_visible_length(tree) as i32)
            .max()
            .unwrap()
    }
}

fn parse_input_to_grid(input: String) -> Grid {
    let mut grid = Grid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let height = char.to_digit(10).unwrap() as i8;

            let coordinate = Coordinate {
                x: x as i32,
                y: y as i32,
            };

            grid.add_tree(Tree { height, coordinate });
        }
    }

    grid
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_8/input.txt");

    let grid = parse_input_to_grid(input);

    println!("Day 8, part 1: {:?}", grid.get_visible_trees_count());

    println!("Day 8, part 2: {:?}", grid.get_max_visible_length());
}
