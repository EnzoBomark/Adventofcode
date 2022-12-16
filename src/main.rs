mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod file_reader;

fn main() {
    use std::time::Instant;
    let before = Instant::now();

    let day = std::env::args().nth(1);

    if let Some(day) = day {
        match day.as_str() {
            "01" => day_1::calorie_counting::run(),
            "02" => day_2::rock_paper_scissors::run(),
            "03" => day_3::rucksack_organization::run(),
            "04" => day_4::camp_cleanup::run(),
            "05" => day_5::supply_stacks::run(),
            "06" => day_6::tuning_trouble::run(),
            "07" => day_7::no_space_left_on_device::run(),
            "08" => day_8::treetop_tree_house::run(),
            _ => println!("Day {} does not exist", day),
        }
    } else {
        println!("No day specified. here are the the answers");
        day_1::calorie_counting::run();
        day_2::rock_paper_scissors::run();
        day_3::rucksack_organization::run();
        day_4::camp_cleanup::run();
        day_5::supply_stacks::run();
        day_6::tuning_trouble::run();
        day_7::no_space_left_on_device::run();
        day_8::treetop_tree_house::run();
        println!("Done 🎉");
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
}
