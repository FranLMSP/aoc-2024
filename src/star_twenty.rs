use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty.txt").unwrap();
    let (positions, w, h) = parse_input(&file);
    let result = calculate_total_map_score(&positions, w, h);

    println!("Result: {}", result);
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1), // Up
    (0, 1), // Down
    (-1, 0), // Left
    (1, 0), // Right
];

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    h: isize,
}

fn parse_input(input: &String) -> (Vec<Position>, usize, usize) {

    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut input = input.clone();
    input.retain(|s| !s.is_whitespace());

    let mut positions = vec![];

    let mut x = 0;
    let mut y = 0;
    for found_char in input.chars() {
        let h = found_char.to_string().parse::<isize>().unwrap();
        positions.push(Position{x, y, h});
        x += 1;
        if x as usize >= w {
            x = 0;
            y += 1;
        }
    }

    (positions, w, h)
}

fn calculate_trailhead_score(current_position: &Position, positions: &Vec<Position>, w: usize, h: usize) -> usize {
    if current_position.h >= 9 {
        return 1;
    }
    let mut count = 0;

    for (index, direction) in DIRECTIONS.iter().enumerate() {
        if current_position.x <= 0 && index == 2
            || current_position.x >= (w as isize - 1) && index == 3
            || current_position.y <= 0 && index == 0
            || current_position.y >= (h as isize - 1) && index == 1
        {
            continue;
        }
        let x = (current_position.x + direction.0) as usize;
        let y = (current_position.y + direction.1) as usize;
        let new_position = positions[x + (y * w)];
        if new_position.h == (current_position.h + 1) {
            count += calculate_trailhead_score(&new_position, positions, w, h);
        }
    }

    count
}

fn calculate_total_map_score(positions: &Vec<Position>, w: usize, h: usize) -> usize {
    positions.iter()
        .filter(|p| p.h == 0)
        .map(|p| calculate_trailhead_score(p, positions, w, h))
        .sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "0123",
            "1234",
            "8765",
            "9876"
        ].join("\n");
        assert_eq!(
            parse_input(&input),
            (vec![
                Position{x: 0, y: 0, h: 0},
                Position{x: 1, y: 0, h: 1},
                Position{x: 2, y: 0, h: 2},
                Position{x: 3, y: 0, h: 3},
                Position{x: 0, y: 1, h: 1},
                Position{x: 1, y: 1, h: 2},
                Position{x: 2, y: 1, h: 3},
                Position{x: 3, y: 1, h: 4},
                Position{x: 0, y: 2, h: 8},
                Position{x: 1, y: 2, h: 7},
                Position{x: 2, y: 2, h: 6},
                Position{x: 3, y: 2, h: 5},
                Position{x: 0, y: 3, h: 9},
                Position{x: 1, y: 3, h: 8},
                Position{x: 2, y: 3, h: 7},
                Position{x: 3, y: 3, h: 6},
            ], 4, 4),
        );
    }

    #[test]
    fn test_calculate_trailhead_score() {
        let input = vec![
            "0000000",
            "0043210",
            "0050020",
            "0065430",
            "0070040",
            "0087650",
            "0090000"
        ].join("\n");
        let (positions, w, h) = parse_input(&input);
        let current_position = Position{x: 5, y: 0, h: 0};
        assert_eq!(calculate_trailhead_score(&current_position, &positions, w, h), 3);

        let input = vec![
            "0090009",
            "0001098",
            "0002007",
            "6543456",
            "7650987",
            "8760000",
            "9870000"
        ].join("\n");
        let (positions, w, h) = parse_input(&input);
        let current_position = Position{x: 3, y: 0, h: 0};
        assert_eq!(calculate_trailhead_score(&current_position, &positions, w, h), 13);

        let input = vec![
            "012345",
            "123456",
            "234567",
            "345678",
            "406789",
            "567890",
        ].join("\n");
        let (positions, w, h) = parse_input(&input);
        let current_position = Position{x: 0, y: 0, h: 0};
        assert_eq!(calculate_trailhead_score(&current_position, &positions, w, h), 227);
    }

    #[test]
    fn test_calculate_total_map_score() {
        let input = vec![
            "89010123",
            "78121874",
            "87430965",
            "96549874",
            "45678903",
            "32019012",
            "01329801",
            "10456732"
        ].join("\n");
        let (positions, w, h) = parse_input(&input);
        assert_eq!(calculate_total_map_score(&positions, w, h), 81);
    }
}
