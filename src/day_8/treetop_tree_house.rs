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
}
