mod day_1;
mod day_2;
mod day_3;
mod file_reader;

fn main() {
    let day = std::env::args().nth(1);

    if let Some(day) = day {
        match day.as_str() {
            "01" => day_1::calorie_counting::run(),
            "02" => day_2::rock_paper_scissors::run(),
            "03" => day_3::rucksack_organization::run(),
            _ => println!("Day {} does not exist", day),
        }
    } else {
        println!("No day specified. here are the the answers");
        day_1::calorie_counting::run();
        day_2::rock_paper_scissors::run();
        day_3::rucksack_organization::run();
        println!("Done 🎉");
    }
}
