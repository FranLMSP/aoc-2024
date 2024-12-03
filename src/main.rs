use std::env;

mod star_one;
mod star_two;
mod star_three;
mod star_four;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => star_one::run(),
        "2" => star_two::run(),
        "3" => star_three::run(),
        "4" => star_four::run(),
        _ => unreachable!(),
    }
}
