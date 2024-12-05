use std::fs;

const XMAS: &str = "MAS";

pub fn run() {
    let file = fs::read_to_string("./inputs/star_eight.txt").unwrap();
    let result = count_all(&file);
    println!("Result: {}", result);
}

fn count_all(input: &String) -> usize {
    let mut count = 0;
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut input = input.clone();
    input.retain(|s| !s.is_whitespace());

    for x in 0..w {
        for y in 0..h {
            let char = input.chars().nth(x + (y * w)).unwrap();
            if char.to_string() == "A" && is_xmas(&input, x, y, w, h) {
                count += 1;
            }
        }
    }

    count
}

fn is_xmas(input: &String, x: usize, y: usize, w: usize, h: usize) -> bool {
    if x < 1 {
        return false;
    }
    if y < 1 {
        return false;
    }
    if w - x <= 1 {
        return false;
    }
    if h - y <= 1 {
        return false;
    }

    // top left to bottom right
    let xmas_1 = format!(
        "{}{}{}",
        get_char_xy(input, x - 1, y - 1, w),
        get_char_xy(input, x,     y    , w),
        get_char_xy(input, x + 1, y + 1, w),
    );

    // bottom left to top right
    let xmas_2 = format!(
        "{}{}{}",
        get_char_xy(input, x - 1, y + 1, w),
        get_char_xy(input, x,     y    , w),
        get_char_xy(input, x + 1, y - 1, w),
    );

    (xmas_1 == XMAS || xmas_1.chars().rev().collect::<String>() == XMAS) &&
    (xmas_2 == XMAS || xmas_2.chars().rev().collect::<String>() == XMAS)

}

fn get_char_xy(input: &String, x: usize, y: usize, w: usize) -> String {
    input.chars().nth(x + (y * w)).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_all() {
        assert_eq!(
            count_all(
                &vec![
                    ".M.S......",
                    "..A..MSMS.",
                    ".M.S.MAA..",
                    "..A.ASMSM.",
                    ".M.S.M....",
                    "..........",
                    "S.S.S.S.S.",
                    ".A.A.A.A..",
                    "M.M.M.M.M.",
                    ".........."
                ].join("\n")
            ),
            9,
        );
    }

    #[test]
    fn test_is_xmas() {
        assert_eq!(
            is_xmas(
                &vec![
                    "M.S",
                    ".A.",
                    "M.S",
                ].join(""),
                1, 1, 3, 3
            ),
            true,
        );
        assert_eq!(
            is_xmas(
                &vec![
                    "M.M",
                    ".A.",
                    "S.S",
                ].join(""),
                1, 1, 3, 3
            ),
            true,
        );
        assert_eq!(
            is_xmas(
                &vec![
                    "S.M",
                    ".A.",
                    "S.M",
                ].join(""),
                1, 1, 3, 3
            ),
            true,
        );
        assert_eq!(
            is_xmas(
                &vec![
                    "S.S",
                    ".A.",
                    "M.M",
                ].join(""),
                1, 1, 3, 3
            ),
            true,
        );
        assert_eq!(
            is_xmas(
                &vec![
                    "M.S",
                    ".A.",
                    "S.M",
                ].join(""),
                1, 1, 3, 3
            ),
            false,
        );
        assert_eq!(
            is_xmas(
                &vec![
                    "S.M",
                    ".A.",
                    "M.S",
                ].join(""),
                1, 1, 3, 3
            ),
            false,
        );
    }

    #[test]
    fn test_get_char_xy() {
        assert_eq!(
            get_char_xy(
                &vec![
                    "M.S",
                    ".A.",
                    "M.S",
                ].join(""),
                0, 0, 3
            ),
            "M",
        );
        assert_eq!(
            get_char_xy(
                &vec![
                    "M.S",
                    ".A.",
                    "M.S",
                ].join(""),
                1, 0, 3
            ),
            ".",
        );
        assert_eq!(
            get_char_xy(
                &vec![
                    "123",
                    "456",
                    "789",
                ].join(""),
                2, 0, 3
            ),
            "3",
        );
        assert_eq!(
            get_char_xy(
                &vec![
                    "123",
                    "456",
                    "789",
                ].join(""),
                0, 1, 3
            ),
            "4",
        );
    }
}
