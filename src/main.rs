use std::env;

mod star_one;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => star_one::run(),
        _ => unreachable!(),
    }
}
