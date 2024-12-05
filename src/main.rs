use std::env;

mod star_one;
mod star_two;
mod star_three;
mod star_four;
mod star_five;
mod star_six;
mod star_seven;
mod star_eight;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => star_one::run(),
        "2" => star_two::run(),
        "3" => star_three::run(),
        "4" => star_four::run(),
        "5" => star_five::run(),
        "6" => star_six::run(),
        "7" => star_seven::run(),
        "8" => star_eight::run(),
        _ => unreachable!(),
    }
}
