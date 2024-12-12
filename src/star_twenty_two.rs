use std::fs;
use cached::proc_macro::cached;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_two.txt").unwrap();
    let result = count_actual_stones(&file, 75);

    println!("Result: {}", result);
}

fn count_actual_stones(stones: &String, blink_count: usize) -> usize {
    stones.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|stone| count_resulting_stones(stone, blink_count))
        .sum()
}

#[cached]
fn count_resulting_stones(stone: usize, blinks: usize) -> usize {
    if blinks <= 0 {
        return 1;
    }
    if stone == 0 {
        return count_resulting_stones(1, blinks - 1);
    }
    let digits = get_digits(stone);
    if digits & 1 == 1 {
        return count_resulting_stones(stone * 2024, blinks - 1);
    }
    let half = digits / 2;
    let pow = 10_usize.pow(half);
    let first_half = stone / pow;
    let second_half = stone % pow;

    count_resulting_stones(first_half, blinks - 1) +
    count_resulting_stones(second_half, blinks - 1)
}

fn get_digits(num: usize) -> u32 {
    let mut digits = 0;
    let mut aux = num;
    while aux >= 1 {
        aux /= 10;
        digits += 1;
    }
    digits
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_actual_stones() {
        assert_eq!(count_actual_stones(&"125 17".to_string(), 1), 3);
        assert_eq!(count_actual_stones(&"125 17".to_string(), 2), 4);
        assert_eq!(count_actual_stones(&"125 17".to_string(), 3), 5);
        assert_eq!(count_actual_stones(&"125 17".to_string(), 4), 9);
        assert_eq!(count_actual_stones(&"125 17".to_string(), 5), 13);
        assert_eq!(count_actual_stones(&"125 17".to_string(), 6), 22);
        assert_eq!(count_actual_stones(&"125 17".to_string(), 25), 55312);
    }

    #[test]
    fn test_count_resulting_stones() {
        assert_eq!(count_resulting_stones(0, 1), 1);
        assert_eq!(count_resulting_stones(1000, 1), 2);
        assert_eq!(count_resulting_stones(1234, 1), 2);
        assert_eq!(count_resulting_stones(1, 1), 1);
    }
}
