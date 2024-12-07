use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_fourteen.txt").unwrap();
    let result = process_input(file.lines());

    println!("Result: {}", result);
}

// result = 204976636995111
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operator {
    Sum,
    Mul,
    Concat,
}

fn compute_formula(expected_result: isize, operands: &Vec<isize>) -> Option<isize> {
    let mut current_operators = initialize_operators(operands.len());
    let max_attempts = 3_usize.pow((operands.len() as u32) - 1);

    assert_eq!(current_operators.len(), operands.len());

    for _ in 0..max_attempts {
        let mut result = 0;
        let mut is_first = true;

        for (operand, operator) in operands.iter().zip(&current_operators) {
            if is_first {
                result = *operand;
                is_first = false;
                continue;
            }
            match operator {
                Operator::Mul => result *= *operand,
                Operator::Sum => result += *operand,
                Operator::Concat => result = format!("{}{}", result, *operand).parse().unwrap(),
            };

            if result > expected_result {
                break;
            }
        }

        if result == expected_result {
            return Some(result);
        }

        cycle_operators(&mut current_operators);

    }

    None
}

fn initialize_operators(len: usize) -> Vec<Operator> {
    vec![Operator::Sum; len]
}

fn cycle_operators(operators: &mut Vec<Operator>) {
    let mut is_done = false;
    let mut current_index = 1;

    // Sum = 0, Mul = 1, Concat = 2
    while !is_done && current_index < operators.len() {
        let operator = operators[current_index];
        let new_operator = match operator {
            Operator::Sum => Operator::Mul,
            Operator::Mul => Operator::Concat,
            Operator::Concat => Operator::Sum,
        };

        operators[current_index] = new_operator;

        is_done = match operator {
            Operator::Sum | Operator::Mul => true,
            Operator::Concat => false,
        };

        if !is_done {
            current_index += 1;
        }

    }
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
        assert_eq!(process_input(input), 11387);
    }

    #[test]
    fn test_cycle_operators() {
        let mut operators = vec![Operator::Sum, Operator::Sum, Operator::Sum];
        cycle_operators(&mut operators);
        assert_eq!(operators, vec![Operator::Sum, Operator::Mul, Operator::Sum]);
        cycle_operators(&mut operators);
        assert_eq!(operators, vec![Operator::Sum, Operator::Concat, Operator::Sum]);
        cycle_operators(&mut operators);
        assert_eq!(operators, vec![Operator::Sum, Operator::Sum, Operator::Mul]);
        cycle_operators(&mut operators);
        assert_eq!(operators, vec![Operator::Sum, Operator::Mul, Operator::Mul]);
        cycle_operators(&mut operators);
        assert_eq!(operators, vec![Operator::Sum, Operator::Concat, Operator::Mul]);
        cycle_operators(&mut operators);
        assert_eq!(operators, vec![Operator::Sum, Operator::Sum, Operator::Concat]);
    }


    #[test]
    fn test_compute_formula() {
        assert_eq!(compute_formula(190, &vec![10, 19]), Some(190));
        assert_eq!(compute_formula(3267, &vec![81, 40, 27]), Some(3267));
        assert_eq!(compute_formula(83, &vec![17, 5]), None);
        assert_eq!(compute_formula(156, &vec![15, 6]), Some(156));
        assert_eq!(compute_formula(7290, &vec![6, 8, 6, 15]), Some(7290));
        assert_eq!(compute_formula(161011, &vec![16, 10, 13]), None);
        assert_eq!(compute_formula(192, &vec![17, 8, 14]), Some(192));
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
