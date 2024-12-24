use std::env;

pub mod day1;
pub mod day2;
pub mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_id: &i8 = &args[1].parse::<i8>().unwrap();
    println!("Selected day: {day_id}");

    match day_id {
        1 => {
            day1::run();
        }
        2 => {
            day2::run();
        }
        3 => {
            day3::run();
        }
        _ => {
            panic!("Unknown day selected: {day_id}");
        }
    }
}
