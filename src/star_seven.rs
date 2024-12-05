use std::fs;

const XMAS: &str = "XMAS";

pub fn run() {
    let file = fs::read_to_string("./inputs/star_seven.txt").unwrap();
    let result = count_all(&file);
    println!("Result: {}", result);
}

fn count_all(input: &String) -> usize {
    count_horizontal(input) +
    count_vertical(input) +
    count_diagonal(input)
}

fn count_diagonal(input: &String) -> usize {
    let mut count = 0;

    // check left to right
    let mut diagonal_string: Vec<String> = vec![];
    for (index, line) in input.lines().enumerate() {
        diagonal_string.push(shift_string_left(&line.to_string(), index));
    }
    count += count_vertical(&diagonal_string.join("\n"));

    // check right to left
    let mut diagonal_string: Vec<String> = vec![];
    for (index, line) in input.lines().enumerate() {
        diagonal_string.push(shift_string_right(&line.to_string(), index));
    }
    count += count_vertical(&diagonal_string.join("\n"));

    count
}

fn shift_string_left(input: &String, count: usize) -> String {
    let len = input.len();
    format!(
        "{}{}{}",
        " ".repeat(len - count),
        input,
        " ".repeat(count),
    )
}

fn shift_string_right(input: &String, count: usize) -> String {
    let len = input.len();
    format!(
        "{}{}{}",
        " ".repeat(count),
        input,
        " ".repeat(len - count),
    )
}

fn count_vertical(input: &String) -> usize {
    count_horizontal(&convert_columns_to_lines(input))
}

fn convert_columns_to_lines(input: &String) -> String {
    let mut columns: Vec<String> = vec![];

    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            match columns.get_mut(index) {
                Some(str) => {
                    str.push(char);
                    columns[index] = str.to_owned();
                },
                None => {
                    columns.push(String::from(char));
                },
            };
        }
    }

    columns.join("\n")
}

fn count_horizontal(input: &String) -> usize {
    let mut count = 0;
    for line in input.lines() {
        count += line.matches(XMAS).count();
        count += &line.chars().rev().collect::<String>().matches(XMAS).count();
    }

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_all() {
        assert_eq!(
            count_all(
                &vec![
                    "....XXMAS.",
                    ".SAMXMS...",
                    "...S..A...",
                    "..A.A.MS.X",
                    "XMASAMX.MM",
                    "X.....XA.A",
                    "S.S.S.S.SS",
                    ".A.A.A.A.A",
                    "..M.M.M.MM",
                    ".X.X.XMASX"
                ].join("\n")
            ),
            18,
        );

        assert_eq!(
            count_all(
                &vec![
                    "MMMSXXMASM",
                    "MSAMXMSMSA",
                    "AMXSXMAAMM",
                    "MSAMASMSMX",
                    "XMASAMXAMM",
                    "XXAMMXXAMA",
                    "SMSMSASXSS",
                    "SAXAMASAAA",
                    "MAMMMXMMMM",
                    "MXMXAXMASX"
                ].join("\n")
            ),
            18,
        );
    }

    #[test]
    fn test_count_diagonal() {
        assert_eq!(
            count_diagonal(
                &vec![
                    "....XXMAS.",
                    ".SAMXMS...",
                    "...S..A...",
                    "..A.A.MS.X",
                    "XMASAMX.MM",
                    "X.....XA.A",
                    "S.S.S.S.SS",
                    ".A.A.A.A.A",
                    "..M.M.M.MM",
                    ".X.X.XMASX"
                ].join("\n")
            ),
            10,
        );
        assert_eq!(
            count_diagonal(
                &vec![
                    "MMMSXXMASM",
                    "MSAMXMSMSA",
                    "AMXSXMAAMM",
                    "MSAMASMSMX",
                    "XMASAMXAMM",
                    "XXAMMXXAMA",
                    "SMSMSASXSS",
                    "SAXAMASAAA",
                    "MAMMMXMMMM",
                    "MXMXAXMASX"
                ].join("\n")
            ),
            10,
        );
    }

    #[test]
    fn test_convert_columns_to_lines() {
        assert_eq!(
            convert_columns_to_lines(
                &vec![
                    "11111",
                    "22222",
                    "33333",
                    "44444",
                    "55555",
                ].join("\n")
            ),
            vec![
                    "12345",
                    "12345",
                    "12345",
                    "12345",
                    "12345",
            ].join("\n")
        )
    }

    #[test]
    fn test_shift_string_left() {
        assert_eq!(shift_string_left(&"12345".to_string(), 0), "     12345");
        assert_eq!(shift_string_left(&"12345".to_string(), 1), "    12345 ");
        assert_eq!(shift_string_left(&"12345".to_string(), 2), "   12345  ");
        assert_eq!(shift_string_left(&"12345".to_string(), 3), "  12345   ");
        assert_eq!(shift_string_left(&"12345".to_string(), 4), " 12345    ");
    }

    #[test]
    fn test_shift_string_right() {
        assert_eq!(shift_string_right(&"12345".to_string(), 0), "12345     ");
        assert_eq!(shift_string_right(&"12345".to_string(), 1), " 12345    ");
        assert_eq!(shift_string_right(&"12345".to_string(), 2), "  12345   ");
        assert_eq!(shift_string_right(&"12345".to_string(), 3), "   12345  ");
        assert_eq!(shift_string_right(&"12345".to_string(), 4), "    12345 ");
    }

    #[test]
    fn test_count_vertical() {
        assert_eq!(
            count_vertical(
                &vec![
                    "....XXMAS.",
                    ".SAMXMS...",
                    "...S..A...",
                    "..A.A.MS.X",
                    "XMASAMX.MM",
                    "X.....XA.A",
                    "S.S.S.S.SS",
                    ".A.A.A.A.A",
                    "..M.M.M.MM",
                    ".X.X.XMASX"
                ].join("\n")
            ),
            3,
        );
    }

    #[test]
    fn test_count_horizontal() {
        assert_eq!(
            count_horizontal(
                &vec![
                    "....XXMAS.",
                    ".SAMXMS...",
                    "...S..A...",
                    "..A.A.MS.X",
                    "XMASAMX.MM",
                    "X.....XA.A",
                    "S.S.S.S.SS",
                    ".A.A.A.A.A",
                    "..M.M.M.MM",
                    ".X.X.XMASX"
                ].join("\n")
            ),
            5,
        );
    }
}
