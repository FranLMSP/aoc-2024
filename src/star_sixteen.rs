use std::{collections::HashMap, fs};
use regex::Regex;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_sixteen.txt").unwrap();
    let (antennas, w, h) = parse_input(&file);
    let result = count_antinodes(&antennas, w, h);

    println!("Result: {}", result);
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
struct Antenna {
    frequency: char,
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
struct Antinode {
    x: isize,
    y: isize,
}

fn parse_input(input: &String) -> (Vec<Antenna>, usize, usize) {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let re = Regex::new(r"[0-9a-zA-Z]").unwrap();

    let mut input = input.clone();
    input.retain(|s| !s.is_whitespace());

    let mut antennas = vec![];

    let mut x = 0;
    let mut y = 0;
    for found_char in input.chars() {
        if re.is_match(&found_char.to_string()) {
            antennas.push(Antenna{frequency: found_char, x: x as isize, y: y as isize})
        }
        x += 1;
        if x >= w {
            x = 0;
            y += 1;
        }
    }

    (antennas, w, h)
}

fn create_antinodes(antenna: &Antenna, target_antenna: &Antenna, w: usize, h: usize) -> Vec<Antinode> {
    if antenna.frequency != target_antenna.frequency {
        return vec![];
    }

    if antenna.y > target_antenna.y {
        return create_antinodes(target_antenna, antenna, w, h);
    }

    let point_x = (target_antenna.x as f64 - antenna.x as f64).powf(2.0);
    let point_y = (target_antenna.y as f64 - antenna.y as f64).powf(2.0);

    let distance = (point_x + point_y * 1.0).sqrt();
    if distance.floor() < 1.0 {
        return vec![];
    }

    let vector = Antinode{
        x: antenna.x - target_antenna.x,
        y: target_antenna.y - antenna.y,
    };

    let mut generated_antinodes = vec![];
    generated_antinodes.push(Antinode { x: antenna.x, y: antenna.y });
    generated_antinodes.push(Antinode { x: target_antenna.x, y: target_antenna.y });

    // generate bottom to top
    let mut is_valid = true;
    let mut last_antinode_x = antenna.x;
    let mut last_antinode_y = antenna.y;
    while is_valid {
        let antinode = Antinode{
            x: last_antinode_x + vector.x,
            y: last_antinode_y - vector.y,
        };
        is_valid = antinode.x >= 0 && antinode.x < w as isize &&
            antinode.y >= 0 && antinode.y < h as isize;

        if is_valid {
            generated_antinodes.push(antinode);
            last_antinode_x = antinode.x;
            last_antinode_y = antinode.y;
        }
    }

    // generate top to bottom
    let mut is_valid = true;
    let mut last_antinode_x = target_antenna.x;
    let mut last_antinode_y = target_antenna.y;
    while is_valid {
        let antinode = Antinode{
            x: last_antinode_x + (vector.x * -1),
            y: last_antinode_y + vector.y,
        };
        is_valid = antinode.x >= 0 && antinode.x < w as isize &&
            antinode.y >= 0 && antinode.y < h as isize;

        if is_valid {
            generated_antinodes.push(antinode);
            last_antinode_x = antinode.x;
            last_antinode_y = antinode.y;
        }
    }

    generated_antinodes
}

fn count_antinodes(antennas: &Vec<Antenna>, w: usize, h: usize) -> usize {
    let mut all_antinodes = vec![];
    for antenna in antennas {
        for target_antenna in antennas {
            let antinodes = create_antinodes(antenna, target_antenna, w, h);
            all_antinodes.extend(antinodes);
        }
    }

    print_antinodes(&all_antinodes, w, h);

    let mut antinodes_map = HashMap::new();
    for antinode in all_antinodes {
        antinodes_map.insert(antinode, true);
    }

    antinodes_map.values().count()
}

fn print_antinodes(antinodes: &Vec<Antinode>, w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let mut char = '.';
            for antinode in antinodes {
                if antinode.x == x as isize && antinode.y == y as isize {
                    char = '#';
                }
            }
            print!("{}", char);
        }
        print!("\n");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_antinodes() {
        let antenna_a = Antenna{frequency: 'T', x: 0, y: 0};
        let antenna_b = Antenna{frequency: 'T', x: 3, y: 1};

        assert_eq!(
            create_antinodes(&antenna_a, &antenna_b, 10, 10),
            vec![Antinode{x: 0, y: 0}, Antinode{x: 3, y: 1}, Antinode{x: 6, y: 2}, Antinode{x: 9, y: 3}],
        );

        let antenna_a = Antenna{frequency: 'T', x: 3, y: 1};
        let antenna_b = Antenna{frequency: 'T', x: 1, y: 2};

        assert_eq!(
            create_antinodes(&antenna_a, &antenna_b, 10, 10),
            vec![Antinode{x: 3, y: 1}, Antinode{x: 1, y: 2}, Antinode{x: 5, y: 0}],
        );
    }

    #[test]
    fn test_count_antinodes() {
        let antennas = vec![
            Antenna{frequency: '0', x: 8, y: 1},
            Antenna{frequency: '0', x: 5, y: 2},
            Antenna{frequency: '0', x: 7, y: 3},
            Antenna{frequency: '0', x: 4, y: 4},
            Antenna{frequency: 'A', x: 6, y: 5},
            Antenna{frequency: 'A', x: 8, y: 8},
            Antenna{frequency: 'A', x: 9, y: 9},
        ];
        assert_eq!(count_antinodes(&antennas, 12, 12), 34);

        let antennas = vec![
            Antenna{frequency: 'T', x: 0, y: 0},
            Antenna{frequency: 'T', x: 3, y: 1},
            Antenna{frequency: 'T', x: 1, y: 2},
        ];
        assert_eq!(count_antinodes(&antennas, 10, 10), 9);
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ].join("\n");
        let expected_result = vec![
            Antenna{frequency: '0', x: 8, y: 1},
            Antenna{frequency: '0', x: 5, y: 2},
            Antenna{frequency: '0', x: 7, y: 3},
            Antenna{frequency: '0', x: 4, y: 4},
            Antenna{frequency: 'A', x: 6, y: 5},
            Antenna{frequency: 'A', x: 8, y: 8},
            Antenna{frequency: 'A', x: 9, y: 9},
        ];
        assert_eq!(parse_input(&input), (expected_result, 12, 12));
    }
}
