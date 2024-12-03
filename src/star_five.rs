use std::fs;
use regex::Regex;


pub fn run() {
    let file = fs::read_to_string("./inputs/star_five.txt").unwrap();
    let input = file.lines();
    let str_instructions = filter_instructions(input);
    let instructions = str_instructions.iter().map(|i| parse_instruction(i)).collect();
    let result = run_instructions(instructions);
    println!("Result: {}", result);
}

fn filter_instructions<'a, I>(str_lines: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a str>
{
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut instructions = vec![];

    for str_line in str_lines {
        instructions.extend(
            re.find_iter(str_line).map(|s| String::from(s.as_str())).into_iter()
        );
    }

    instructions
}

fn parse_instruction(instruction: &str) -> (isize, isize) {
    let re = Regex::new(r"[0-9]{1,3},[0-9]{1,3}").unwrap();
    let substring = re.find(instruction).unwrap().as_str();
    let nums: Vec<isize> = substring.split(",").map(|s| s.to_string().parse::<isize>().unwrap()).collect();
    (nums[0], nums[1])
}

fn run_instructions(instructions: Vec<(isize, isize)>) -> isize {
    instructions.iter().map(|&e| e.0 * e.1).reduce(|a, b| a + b).unwrap_or(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_instructions() {
        assert_eq!(
            filter_instructions(vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]),
            vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"],
        )
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("mul(2,4)"), (2, 4));
        assert_eq!(parse_instruction("mul(5,5)"), (5, 5));
        assert_eq!(parse_instruction("mul(11,8)"), (11, 8));
        assert_eq!(parse_instruction("mul(8,5)"), (8, 5));
    }

    #[test]
    fn test_run_instructions() {
        assert_eq!(run_instructions(vec![(2, 4), (5, 5), (11, 8), (8, 5)]), 161);
        assert_eq!(run_instructions(vec![]), 0);
    }
}
