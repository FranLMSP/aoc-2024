use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_one.txt").unwrap();
    let result = count_actual_stones(&file);

    println!("Result: {}", result);
}

fn count_actual_stones(stones: &String) -> usize {
    blink(stones, 25).split_whitespace().count()
}

fn blink(stones: &String, count: usize) -> String {
    let mut new_stones = stones.clone();
    for _ in 0..count {
        new_stones = do_blink(&new_stones);
    }

    new_stones
}


fn do_blink(string: &String) -> String {
    let stones: Vec<String> = string.split_whitespace().map(|s| s.to_string()).collect();
    let mut new_stones = vec![];
    for stone in stones {
        new_stones.extend(apply_rules(&stone));
    }

    new_stones.join(" ")
}


fn apply_rules(stone: &String) -> Vec<String> {
    let len = stone.len();
    if stone == "0" {
        return vec!["1".to_string()];
    } else if len & 1 == 0 { // check if str len is an even number
        return vec![
            format!("{}", stone[0..(len / 2)].to_string().parse::<usize>().unwrap()),
            format!("{}", stone[(len / 2)..].to_string().parse::<usize>().unwrap()),
        ];
    }
    vec![format!("{}", stone.parse::<usize>().unwrap_or(0) * 2024)]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_actual_stones() {
        assert_eq!(count_actual_stones(&"125 17".to_string()), 55312);
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink(&"125 17".to_string(), 6), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
    }

    #[test]
    fn test_do_blink() {
        assert_eq!(do_blink(&"0 1 10 99 999".to_string()), "1 2024 1 0 9 9 2021976");
        assert_eq!(do_blink(&"125 17".to_string()), "253000 1 7");
        assert_eq!(do_blink(&"253000 1 7".to_string()), "253 0 2024 14168");
        assert_eq!(do_blink(&"253 0 2024 14168".to_string()), "512072 1 20 24 28676032");
        assert_eq!(do_blink(&"512072 1 20 24 28676032".to_string()), "512 72 2024 2 0 2 4 2867 6032");
        assert_eq!(do_blink(&"512 72 2024 2 0 2 4 2867 6032".to_string()), "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
        assert_eq!(do_blink(&"1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32".to_string()), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
    }

    #[test]
    fn test_apply_rules() {
        assert_eq!(apply_rules(&"0".to_string()), vec!["1".to_string()]);
        assert_eq!(apply_rules(&"1000".to_string()), vec!["10".to_string(), "0".to_string()]);
        assert_eq!(apply_rules(&"1".to_string()), vec!["2024".to_string()]);
    }
}
