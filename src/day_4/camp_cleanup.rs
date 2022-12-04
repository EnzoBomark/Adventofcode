use crate::file_reader;

struct Sections {
    start: i32,
    end: i32,
}

struct CleaningSections {
    first: Sections,
    second: Sections,
}

fn section_str_to_range(s: &str) -> Sections {
    let range = s
        .split('-')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&val| val.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Sections {
        start: range[0],
        end: range[1],
    }
}

fn print_fully_overlapping_sections(sections: &Vec<CleaningSections>) {
    let mut number_of_fully_overlapping_sections = 0;

    for section in sections {
        let first_section_includes_second_section =
            section.first.start <= section.second.start && section.first.end >= section.second.end;

        let second_section_includes_first_section =
            section.second.start <= section.first.start && section.second.end >= section.first.end;

        if first_section_includes_second_section || second_section_includes_first_section {
            number_of_fully_overlapping_sections += 1;
        }
    }

    println!("Day 04: Part 1: {}", number_of_fully_overlapping_sections);
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_4/input.txt");

    let cleaning_sections_str: Vec<&str> = input.split('\n').collect();

    let cleaning_sections: Vec<CleaningSections> = cleaning_sections_str
        .iter()
        .map(|&val| {
            let sections = val.split(',').collect::<Vec<&str>>();

            let first_assignment_range = section_str_to_range(sections[0]);
            let second_assignment_range = section_str_to_range(sections[1]);

            CleaningSections {
                first: first_assignment_range,
                second: second_assignment_range,
            }
        })
        .collect::<Vec<CleaningSections>>();

    print_fully_overlapping_sections(&cleaning_sections);
}
