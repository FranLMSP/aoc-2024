use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_thirteen.txt").unwrap();
    let result = process_input(file.lines());

    println!("Result: {}", result);
}

// 13320722 is too low
fn process_input<'a, I>(str_lines: I) -> isize 
where
    I: IntoIterator<Item = &'a str>
{
    str_lines.into_iter()
        .map(|s| parse_formula(&s.to_string()))
        .map(|(expected_result, operands)| compute_formula(expected_result, &operands))
        .filter(|result| result.is_some())
        .map(|result| result.unwrap())
        .reduce(|acc, e| acc + e)
        .unwrap_or(0)
}

fn parse_formula(str_formula: &String) -> (isize, Vec<isize>) {
    let mut split = str_formula.split(":");
    let expected_result = split
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let nums = split
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    (expected_result, nums)
}

#[derive(Copy, Clone)]
enum Operator {
    Mul,
    Sum,
}

fn compute_formula(expected_result: isize, operands: &Vec<isize>) -> Option<isize> {
    let mut current_mask: u16 = 0b00000000;
    let max_attempts = 2_usize.pow((operands.len() as u32) - 1);

    for _ in 0..max_attempts {
        let mut result = 0;
        let mut is_first = true;

        let operators = mask_to_operators(current_mask, operands.len());

        for (operand, operator) in operands.iter().zip(operators) {
            if is_first {
                result = *operand;
                is_first = false;
                continue;
            }
            match operator {
                Operator::Mul => {result *= *operand},
                Operator::Sum => {result += *operand},
            };
        }


        if result == expected_result {
            return Some(result);
        }

        current_mask += 1;
    }

    None
}

fn mask_to_operators(mask: u16, count: usize) -> Vec<Operator> {
    let mut operators = vec![];
    operators.push(Operator::Sum); // push dummy operator just so that the len matches with operands
    let mut mask_temp = mask;
    for _ in 0..count {
        let operator = match mask_temp & 1 {
            0 => Operator::Sum,
            1 => Operator::Mul,
            _ => unreachable!(),
        };
        operators.push(operator);
        mask_temp >>= 1;
    }

    operators
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ];
        assert_eq!(process_input(input), 3749);
    }

    #[test]
    fn test_compute_formula() {
        assert_eq!(compute_formula(190, &vec![10, 19]), Some(190));
        assert_eq!(compute_formula(3267, &vec![81, 40, 27]), Some(3267));
        assert_eq!(compute_formula(83, &vec![17, 5]), None);
        assert_eq!(compute_formula(156, &vec![15, 6]), None);
        assert_eq!(compute_formula(7290, &vec![6, 8, 6, 15]), None);
        assert_eq!(compute_formula(161011, &vec![16, 10, 13]), None);
        assert_eq!(compute_formula(192, &vec![17, 8, 14]), None);
        assert_eq!(compute_formula(21037, &vec![9, 7, 18, 13]), None);
        assert_eq!(compute_formula(292, &vec![11, 6, 16, 20]), Some(292));
    }

    #[test]
    fn test_parse_formula() {
        assert_eq!(parse_formula(&"190: 10 19".to_string()), (190, vec![10, 19]));
        assert_eq!(parse_formula(&"3267: 81 40 27".to_string()), (3267, vec![81, 40, 27]));
        assert_eq!(parse_formula(&"83: 17 5".to_string()), (83, vec![17, 5]));
        assert_eq!(parse_formula(&"156: 15 6".to_string()), (156, vec![15, 6]));
        assert_eq!(parse_formula(&"7290: 6 8 6 15".to_string()), (7290, vec![6, 8, 6, 15]));
        assert_eq!(parse_formula(&"161011: 16 10 13".to_string()), (161011, vec![16, 10, 13]));
        assert_eq!(parse_formula(&"192: 17 8 14".to_string()), (192, vec![17, 8, 14]));
        assert_eq!(parse_formula(&"21037: 9 7 18 13".to_string()), (21037, vec![9, 7, 18, 13]));
        assert_eq!(parse_formula(&"292: 11 6 16 20".to_string()), (292, vec![11, 6, 16, 20]));
    }
}
