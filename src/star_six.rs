use std::fs;
use std::cmp;
use regex::Regex;

const STR_DO: &str = "do()";
const STR_DONT: &str = "don't()";

pub fn run() {
    let file = fs::read_to_string("./inputs/star_six.txt").unwrap();
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
    let instr_str_len = "mul(123,123)".len();

    let mut instructions = vec![];
    let mut is_enabled = true;

    for str_line in str_lines {
        let mut i = 0;
        while i < (str_line.len() - STR_DONT.len()) {
            let str_slice = str_line[i..(i + STR_DONT.len())].to_string();
            if str_slice.starts_with(STR_DONT) {
                i += STR_DONT.len();
                is_enabled = false;
            } else if str_slice.starts_with(STR_DO) {
                i += STR_DO.len();
                is_enabled = true;
            }

            if !is_enabled {
                i += 1;
                continue;
            }

            let remaining_text = str_line.len() - i;
            let last_char = i + cmp::min(instr_str_len, remaining_text);
            let instr_slice = str_line[i..last_char].to_string();
            let found_instr = re.find(&instr_slice);
            if let Some(found_instr_str) = found_instr {
                let effective_str = found_instr_str.as_str().to_string();
                i += effective_str.len();
                instructions.push(effective_str);
            } else {
                i += 1;
            }
        }
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
            filter_instructions(vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]),
            vec!["mul(2,4)", "mul(8,5)"],
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
