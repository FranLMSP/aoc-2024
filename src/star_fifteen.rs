use std::{collections::HashMap, fs};
use regex::Regex;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_fifteen.txt").unwrap();
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

fn create_antinodes(antenna: &Antenna, target_antenna: &Antenna) -> Vec<Antinode> {
    if antenna.frequency != target_antenna.frequency {
        return vec![];
    }

    if antenna.y > target_antenna.y {
        return create_antinodes(target_antenna, antenna);
    }

    let point_x = (target_antenna.x as f64 - antenna.x as f64).powf(2.0);
    let point_y = (target_antenna.y as f64 - antenna.y as f64).powf(2.0);

    let distance = (point_x + point_y * 1.0).sqrt();
    if distance.floor() < 1.0 {
        return vec![];
    }

    let vector = Antinode{
        x: antenna.x - target_antenna.x,
        y: antenna.y - target_antenna.y,
    };

    vec![
        Antinode{
            x: antenna.x + vector.x,
            y: antenna.y + vector.y,
        },
        Antinode{
            x: antenna.x - (vector.x * 2),
            y: antenna.y - (vector.y * 2),
        },
    ]
}

fn count_antinodes(antennas: &Vec<Antenna>, w: usize, h: usize) -> usize {
    let mut all_antinodes = vec![];
    for antenna in antennas {
        for target_antenna in antennas {
            let antinodes = create_antinodes(antenna, target_antenna);
            all_antinodes.extend(antinodes);
        }
    }

    print_antinodes(&all_antinodes, w, h);

    let mut antinodes_map = HashMap::new();
    for antinode in all_antinodes {
        if
            antinode.x >= (w as isize) || antinode.y >= (h as isize) ||
            antinode.x < 0 || antinode.y < 0
        {
            continue;
        }
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
        let antenna_a = Antenna{frequency: 'a', x: 4, y: 3};
        let antenna_b = Antenna{frequency: 'a', x: 5, y: 5};

        assert_eq!(
            create_antinodes(&antenna_a, &antenna_b),
            vec![Antinode{x: 3, y: 1}, Antinode{x: 6, y: 7}],
        );

        let antenna_a = Antenna{frequency: 'a', x: 5, y: 3};
        let antenna_b = Antenna{frequency: 'a', x: 4, y: 5};

        assert_eq!(
            create_antinodes(&antenna_a, &antenna_b),
            vec![Antinode{x: 6, y: 1}, Antinode{x: 3, y: 7}],
        );

        let antenna_a = Antenna{frequency: '0', x: 8, y: 1};
        let antenna_b = Antenna{frequency: '0', x: 5, y: 2};

        assert_eq!(
            create_antinodes(&antenna_a, &antenna_b),
            vec![Antinode{x: 11, y: 0}, Antinode{x: 2, y: 3}],
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
        assert_eq!(count_antinodes(&antennas, 12, 12), 14);
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
