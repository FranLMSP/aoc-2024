use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_four.txt").unwrap();
    let input = file.lines();
    let reports = parse_input(input);
    let result = count_safe_reports(reports);
    println!("Result: {}", result);
}

fn parse_input<'a, I>(str_lines: I) -> Vec<Vec<isize>>
where
    I: IntoIterator<Item = &'a str>
{
    let mut reports = vec![];

    for str_line in str_lines {
        let parsed_elems: Vec<isize> = str_line.split_whitespace().map(|n| n.to_string().parse::<isize>().unwrap()).collect();
        reports.push(parsed_elems)
    }

    reports
}

fn count_safe_reports(reports: Vec<Vec<isize>>) -> usize {
    reports.iter().filter(|&report| is_report_change_safe_within_tolerance(report)).count()
}

#[derive(PartialEq, Copy, Clone)]
enum Balancing {
    Increasing,
    Decreasing,
}
fn is_report_change_safe_within_tolerance(report: &Vec<isize>) -> bool {
    let is_safe = is_report_safe(report);
    if is_safe {
        return true;
    }

    for index_to_remove in 0..report.len() {
        let filtered_report: Vec<isize> = report
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i != index_to_remove)
            .map(|(_, e)| *e)
            .collect();
        let is_safe_within_tolerance = is_report_safe(&filtered_report);
        if is_safe_within_tolerance {
            return true;
        }
    }

    false
}

fn is_report_safe(report: &Vec<isize>) -> bool {
    if report.len() < 2 {
        return false;
    }
    let mut i = 1;
    let mut last_balancing = None;
    while i < report.len() {
        let prev_level = report[i - 1];
        let current_level = report[i];

        if prev_level == current_level {
            return false;
        } 

        let difference = match current_level > prev_level {
            true => current_level - prev_level,
            false => prev_level - current_level,
        };

        if difference > 3 {
            return false;
        }

        if let Some(current_balancing) = last_balancing {
            let new_balancing = match current_level > prev_level {
                true => Balancing::Increasing,
                false => Balancing::Decreasing,
            };
            if new_balancing != current_balancing {
                return false;
            }
        } else {
            last_balancing = match current_level > prev_level {
                true => Some(Balancing::Increasing),
                false => Some(Balancing::Decreasing),
            }
        }

        i += 1;
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];
        let result = parse_input(input);
        let expected_result = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_count_safe_reports() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(count_safe_reports(reports), 4);
    }

    #[test]
    fn test_is_safe_within_tolerance() {
        assert!(is_report_change_safe_within_tolerance(&vec![7, 6, 4, 2, 1]));
        assert!(!is_report_change_safe_within_tolerance(&vec![1, 2, 7, 8, 9]));
        assert!(!is_report_change_safe_within_tolerance(&vec![9, 7, 6, 2, 1]));
        assert!(is_report_change_safe_within_tolerance(&vec![1, 3, 2, 4, 5]));
        assert!(is_report_change_safe_within_tolerance(&vec![8, 6, 4, 4, 1]));
        assert!(is_report_change_safe_within_tolerance(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_is_level_change_safe() {
        assert!(is_report_safe(&vec![7, 6, 4, 2, 1]));
        assert!(!is_report_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!is_report_safe(&vec![9, 7, 6, 2, 1]));
        assert!(!is_report_safe(&vec![1, 3, 2, 4, 5]));
        assert!(!is_report_safe(&vec![8, 6, 4, 4, 1]));
        assert!(is_report_safe(&vec![1, 3, 6, 7, 9]));
    }
}
