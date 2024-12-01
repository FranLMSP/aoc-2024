use std::env;

mod star_one;
mod star_two;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => star_one::run(),
        "2" => star_two::run(),
        _ => unreachable!(),
    }
}
