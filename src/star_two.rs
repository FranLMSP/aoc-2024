use std::collections::HashMap;

use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_two.txt").unwrap();
    let input = file.lines();
    let parsed_input = parse_input(input);
    let result = calculate_similarity_score(parsed_input.0, parsed_input.1);
    println!("Result: {}", result);
}

fn parse_input<'a, I>(str_lines: I) -> (Vec<isize>, Vec<isize>)
where
    I: IntoIterator<Item = &'a str>
{
    let mut list_one = vec![];
    let mut list_two = vec![];

    for str_line in str_lines {
        if str_line.trim().len() < 1 {
            continue;
        }
        let mut elements = str_line.split_whitespace();
        let val_one: isize = elements.next().unwrap().parse().unwrap();
        let val_two: isize = elements.last().unwrap().parse().unwrap();
        list_one.push(val_one);
        list_two.push(val_two);
    }

    (list_one, list_two)
}

fn calculate_similarity_score(list_one: Vec<isize>, list_two: Vec<isize>) -> isize {
    let mut result = 0;
    let mut id_count = HashMap::new();

    for id in list_two.iter() {
        let current_count = id_count.entry(id).or_insert(0);
        *current_count += 1;
    }

    for id in list_one.iter() {
        let count = match id_count.get(id) {
            Some(count) => *count,
            None => 0,
        };

        result += *id * count;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "3   4",
            "4   3",
            "2   5",
            "1   3",
            "3   9",
            "3   3",
        ];
        let result = parse_input(input);
        assert_eq!(result, (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn test_calculate_similarity_score() {
        let list_one = vec![3, 4, 2, 1, 3, 3];
        let list_two = vec![4, 3, 5, 3, 9, 3];
        let result = calculate_similarity_score(list_one, list_two);
        assert_eq!(result, 31);
    }
}
