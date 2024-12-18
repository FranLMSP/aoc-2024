use std::env;

mod star_one;
mod star_two;
mod star_three;
mod star_four;
mod star_five;
mod star_six;
mod star_seven;
mod star_eight;
mod star_nine;
mod star_ten;
mod star_eleven;
mod star_twelve;
mod star_thirteen;
mod star_fourteen;
mod star_fifteen;
mod star_sixteen;
mod star_seventeen;
mod star_eighteen;
mod star_nineteen;
mod star_twenty;
mod star_twenty_one;
mod star_twenty_two;
mod star_twenty_three;
mod star_twenty_four;
mod star_twenty_five;
mod star_twenty_six;
mod star_twenty_seven;
mod star_twenty_eight;
mod star_twenty_nine;
mod star_thirty;
mod star_thirty_one;
mod star_thirty_two;
mod star_thirty_three;

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
        "9" => star_nine::run(),
        "10" => star_ten::run(),
        "11" => star_eleven::run(),
        "12" => star_twelve::run(),
        "13" => star_thirteen::run(),
        "14" => star_fourteen::run(),
        "15" => star_fifteen::run(),
        "16" => star_sixteen::run(),
        "17" => star_seventeen::run(),
        "18" => star_eighteen::run(),
        "19" => star_nineteen::run(),
        "20" => star_twenty::run(),
        "21" => star_twenty_one::run(),
        "22" => star_twenty_two::run(),
        "23" => star_twenty_three::run(),
        "24" => star_twenty_four::run(),
        "25" => star_twenty_five::run(),
        "26" => star_twenty_six::run(),
        "27" => star_twenty_seven::run(),
        "28" => star_twenty_eight::run(),
        "29" => star_twenty_nine::run(),
        "30" => star_thirty::run(),
        "31" => star_thirty_one::run(),
        "32" => star_thirty_two::run(),
        "33" => star_thirty_three::run(),
        _ => unreachable!(),
    }
}
