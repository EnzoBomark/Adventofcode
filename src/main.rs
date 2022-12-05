mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod file_reader;

fn main() {
    let day = std::env::args().nth(1);

    if let Some(day) = day {
        match day.as_str() {
            "01" => day_1::calorie_counting::run(),
            "02" => day_2::rock_paper_scissors::run(),
            "03" => day_3::rucksack_organization::run(),
            "04" => day_4::camp_cleanup::run(),
            "05" => day_5::supply_stacks::run(),
            _ => println!("Day {} does not exist", day),
        }
    } else {
        println!("No day specified. here are the the answers");
        day_1::calorie_counting::run();
        day_2::rock_paper_scissors::run();
        day_3::rucksack_organization::run();
        day_4::camp_cleanup::run();
        day_5::supply_stacks::run();
        println!("Done 🎉");
    }
}
