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

fn map_cleaning_range_str_to_cleaning_sections(s: &str) -> CleaningSections {
    let sections = s.split(',').collect::<Vec<&str>>();

    let first_assignment_range = section_str_to_range(sections[0]);
    let second_assignment_range = section_str_to_range(sections[1]);

    CleaningSections {
        first: first_assignment_range,
        second: second_assignment_range,
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

fn print_partly_overlapping_sections(sections: &Vec<CleaningSections>) {
    let mut number_of_partly_overlapping_sections = 0;

    for section in sections {
        let first_range = section.first.start..section.first.end;
        let second_range = section.second.start..section.second.end;

        let first_section_includes_second_section = first_range.contains(&section.second.start)
            || first_range.contains(&section.second.end);

        let second_section_includes_first_section = second_range.contains(&section.first.start)
            || second_range.contains(&section.first.end);

        let starts_match = match section.first.start {
            x if x == section.second.start => true,
            x if x == section.second.end => true,
            _ => false,
        };

        let ends_match = match section.first.end {
            x if x == section.second.start => true,
            x if x == section.second.end => true,
            _ => false,
        };

        let sections_partly_overlap = first_section_includes_second_section
            || second_section_includes_first_section
            || starts_match
            || ends_match;

        if sections_partly_overlap {
            number_of_partly_overlapping_sections += 1;
        }
    }

    println!("Day 04: Part 2: {}", number_of_partly_overlapping_sections);
}

pub fn run() {
    let input = file_reader::read_file_in_cwd("src/day_4/input.txt");

    let cleaning_sections_str: Vec<&str> = input.split('\n').collect();

    let cleaning_sections: Vec<CleaningSections> = cleaning_sections_str
        .iter()
        .map(|&val| map_cleaning_range_str_to_cleaning_sections(val))
        .collect::<Vec<CleaningSections>>();

    print_fully_overlapping_sections(&cleaning_sections);

    print_partly_overlapping_sections(&cleaning_sections);
}
