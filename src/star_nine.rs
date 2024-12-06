use std::fs;
use regex::Regex;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_nine.txt").unwrap();
    let input = file.lines();
    let (rules, updates) = parse_input(input);
    let valid_updates = get_valid_updates(&rules, &updates);
    let result = sum_updates_mid_numbers(&valid_updates);
    println!("Result: {}", result);
}

fn parse_input<'a, I>(str_lines: I) -> (Vec<(isize, isize)>, Vec<Vec<isize>>)
where
    I: IntoIterator<Item = &'a str>
{
    let mut rules = vec![];
    let mut updates = vec![];
    for str_line in str_lines {
        if let Some(rule) = parse_rule(&str_line) {
            rules.push(rule);
        }
        let update = parse_update(&str_line);
        if !update.is_empty() {
            updates.push(update);
        }
    }

    (rules, updates)
}

fn sum_updates_mid_numbers(valid_updates: &Vec<Vec<isize>>) -> isize {
    let mut result = 0;
    for update in valid_updates {
        let mid_num = match update.get(update.len().div_ceil(2) - 1) {
            Some(num) => *num,
            None => 0,
        };
        result += mid_num;
    }
    result
}

fn get_valid_updates(rules: &Vec<(isize, isize)>, updates: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut valid_updates = vec![];

    for update in updates {
        if is_update_valid(&rules, update) {
            valid_updates.push(update.clone());
        }
    }

    valid_updates
}

fn is_update_valid(rules: &Vec<(isize, isize)>, update: &Vec<isize>) -> bool {

    for (rule_x, rule_y) in rules {
        let page_x_index = update.iter().position(|v| *v == *rule_x);
        if page_x_index.is_none() {
            continue;
        }
        let page_y_index = update.iter().position(|v| *v == *rule_y);
        if page_y_index.is_none() {
            continue;
        }

        if page_x_index.unwrap() > page_y_index.unwrap() {
            return false
        }
    }

    true
}

fn parse_rule(line: &str) -> Option<(isize, isize)> {
    let re = Regex::new(r"^[0-9]{1,}\|[0-9]{1,}$").unwrap();
    if !re.is_match(line) {
        return None
    }

    let mut segments = line.split("|");
    let left: isize = segments.next().unwrap().parse().unwrap();
    let right: isize = segments.next().unwrap().parse().unwrap();

    Some((left, right))
}

fn parse_update(line: &str) -> Vec<isize> {
    let mut result = vec![];
    for page in line.split(",") {
        let maybe_num = page.parse::<isize>();
        if maybe_num.is_ok() {
            result.push(maybe_num.unwrap());
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let expected_rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let expected_updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ];
        let input_text = vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ].join("\n");
        assert_eq!(
            parse_input(input_text.lines()),
            (expected_rules, expected_updates)
        );
    }

    #[test]
    fn test_is_update_valid() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        assert!(is_update_valid(&rules, &vec![75,47,61,53,29]));
        assert!(is_update_valid(&rules, &vec![97,61,53,29,13]));
        assert!(is_update_valid(&rules, &vec![75,29,13]));
        assert!(!is_update_valid(&rules, &vec![75,97,47,61,53]));
        assert!(!is_update_valid(&rules, &vec![61,13,29]));
        assert!(!is_update_valid(&rules, &vec![97,13,75,29,47]));
    }

    #[test]
    fn test_get_valid_updates() {
        let rules = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];
        let updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ];
        assert_eq!(
            get_valid_updates(&rules, &updates),
            vec![
                vec![75,47,61,53,29],
                vec![97,61,53,29,13],
                vec![75,29,13],
            ],
        )
    }

    #[test]
    fn test_sum_updates_mid_numbers() {
        let valid_updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
        ];
        assert_eq!(
            sum_updates_mid_numbers(&valid_updates),
            143,
        )
    }

    #[test]
    fn test_parse_rule() {
        assert_eq!(parse_rule("12|34"), Some((12, 34)));
        assert_eq!(parse_rule(""), None);
        assert_eq!(parse_rule("12,34,56"), None);
    }

    #[test]
    fn test_parse_update() {
        assert_eq!(parse_update("12|34"), vec![]);
        assert_eq!(parse_update(""), vec![]);
        assert_eq!(parse_update("12,34,56"), vec![12, 34, 56]);
    }
}
