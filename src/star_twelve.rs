use std::{collections::HashMap, fs};

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twelve.txt").unwrap();
    let (guard, obstructions, w, h) = parse_input(&file);
    let result = count_loops(&guard, &obstructions, w, h);
    println!("Result: {}", result);
}

fn parse_input(input: &String) -> (Guard, Vec<Obstruction>, usize, usize) {

    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();

    let mut input = input.clone();
    input.retain(|s| !s.is_whitespace());

    let mut guard = Guard{x: 0, y: 0, direction: GuardDirection::Up};
    let mut obstructions = vec![];

    for y in 0..h {
        for x in 0..w {
            let char = input.chars().nth(x + (y * w)).unwrap().to_string();
            if char == "#" {
                obstructions.push(
                    Obstruction{x, y}
                );
            }

            if "^v<>".contains(&char) {
                guard.x = x;
                guard.y = y;

                guard.direction = match char.chars().next().unwrap() {
                    '^' => GuardDirection::Up,
                    'v' => GuardDirection::Down,
                    '<' => GuardDirection::Left,
                    '>' => GuardDirection::Right,
                    _ => unreachable!(),
                }
            }
        }
    }

    (guard, obstructions, w, h)
}

fn count_loops(guard: &Guard, obstructions: &Vec<Obstruction>, w: usize, h: usize) -> usize {
    let mut loop_count = 0;

    for y in 0..h {
        for x in 0..w {
            let mut virtual_guard = Guard{x: guard.x, y: guard.y, direction: guard.direction};
            let mut visited_positions = HashMap::new();
            let mut obstacle_encounters= HashMap::new();
            let mut virtual_obstructions = obstructions.clone();
            virtual_obstructions.push(Obstruction {x, y});
            while !virtual_guard.can_escape(w, h) {
                let pos = (virtual_guard.x, virtual_guard.y);
                visited_positions.insert(pos, virtual_guard.direction);

                if virtual_guard.is_obstructed(&virtual_obstructions) {
                    let obstacle_encounter = (
                        virtual_guard.x,
                        virtual_guard.y,
                        virtual_guard.direction
                    );
                    match obstacle_encounters.get(&obstacle_encounter) {
                        Some(_) => {
                            loop_count += 1;
                            break;
                        },
                        None => {
                            obstacle_encounters.insert(obstacle_encounter, true)
                        }
                    };
                    virtual_guard.rotate_right();
                    continue;
                }

                virtual_guard.step();

                visited_positions.insert((virtual_guard.x, virtual_guard.y), virtual_guard.direction);
            }

        }
    }

    loop_count
}


#[derive(PartialEq, Debug, Copy, Clone)]
struct Obstruction {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
struct Guard {
    x: usize,
    y: usize,
    direction: GuardDirection,
}

impl Guard {
    pub fn can_escape(&self, w: usize, h: usize) -> bool {
        if self.direction == GuardDirection::Left && self.x == 0 {
            return true;
        }
        if self.direction == GuardDirection::Up && self.y == 0 {
            return true;
        }
        if self.direction == GuardDirection::Right && self.x == w.saturating_sub(1) {
            return true;
        }
        if self.direction == GuardDirection::Down && self.y == h.saturating_sub(1) {
            return true;
        }
        false
    }

    pub fn rotate_right(&mut self) {
        self.direction = match self.direction {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }

    pub fn step(&mut self) {
        match self.direction {
            GuardDirection::Up => {
                if self.y > 0 {
                    self.y -= 1;
                }
            },
            GuardDirection::Right => {
                self.x += 1;
            },
            GuardDirection::Down => {
                self.y += 1;
            },
            GuardDirection::Left => {
                self.x -= 1;
            },
        };
    }

    pub fn is_obstructed(&self, obstructions: &Vec<Obstruction>) -> bool {
        let mut virtual_guard = Guard {x: self.x, y: self.y, direction: self.direction};
        virtual_guard.step();
        for obstruction in obstructions {
            if obstruction.x == virtual_guard.x && obstruction.y == virtual_guard.y {
                return true;
            }
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ].join("\n");
        assert_eq!(
            parse_input(&input),
            (
                Guard{x: 4, y: 6, direction: GuardDirection::Up},
                vec![
                    Obstruction{x: 4, y: 0},
                    Obstruction{x: 9, y: 1},
                    Obstruction{x: 2, y: 3},
                    Obstruction{x: 7, y: 4},
                    Obstruction{x: 1, y: 6},
                    Obstruction{x: 8, y: 7},
                    Obstruction{x: 0, y: 8},
                    Obstruction{x: 6, y: 9},
                ],
                10, 10
            )
        );
    }

    #[test]
    fn test_count_loops() {
        let input = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ].join("\n");
        let (guard, obstructions, _, _) = parse_input(&input);
        assert_eq!(count_loops(&guard, &obstructions, 10, 10), 6);
    }

    #[test]
    fn test_can_guard_escape() {
        let guard = Guard{x: 0, y: 0, direction: GuardDirection::Left};
        assert!(guard.can_escape(10, 10));
        let guard = Guard{x: 0, y: 0, direction: GuardDirection::Up};
        assert!(guard.can_escape(10, 10));
        let guard = Guard{x: 9, y: 0, direction: GuardDirection::Right};
        assert!(guard.can_escape(10, 10));
        let guard = Guard{x: 9, y: 9, direction: GuardDirection::Down};
        assert!(guard.can_escape(10, 10));

        let guard = Guard{x: 5, y: 5, direction: GuardDirection::Left};
        assert!(!guard.can_escape(10, 10));
        let guard = Guard{x: 5, y: 5, direction: GuardDirection::Up};
        assert!(!guard.can_escape(10, 10));
        let guard = Guard{x: 5, y: 5, direction: GuardDirection::Right};
        assert!(!guard.can_escape(10, 10));
        let guard = Guard{x: 5, y: 5, direction: GuardDirection::Down};
        assert!(!guard.can_escape(10, 10));
    }

    #[test]
    fn test_guard_rotate_right() {
        let mut guard = Guard{x: 0, y: 0, direction: GuardDirection::Up};
        guard.rotate_right();
        assert_eq!(guard.direction, GuardDirection::Right);
        let mut guard = Guard{x: 0, y: 0, direction: GuardDirection::Right};
        guard.rotate_right();
        assert_eq!(guard.direction, GuardDirection::Down);
        let mut guard = Guard{x: 0, y: 0, direction: GuardDirection::Down};
        guard.rotate_right();
        assert_eq!(guard.direction, GuardDirection::Left);
        let mut guard = Guard{x: 0, y: 0, direction: GuardDirection::Left};
        guard.rotate_right();
        assert_eq!(guard.direction, GuardDirection::Up);
    }

    #[test]
    fn test_step_guard() {
        let mut guard = Guard{x: 2, y: 2, direction: GuardDirection::Up};
        guard.step();
        assert_eq!(guard, Guard{x: 2, y: 1, direction: GuardDirection::Up});

        let mut guard = Guard{x: 2, y: 2, direction: GuardDirection::Right};
        guard.step();
        assert_eq!(guard, Guard{x: 3, y: 2, direction: GuardDirection::Right});

        let mut guard = Guard{x: 2, y: 2, direction: GuardDirection::Down};
        guard.step();
        assert_eq!(guard, Guard{x: 2, y: 3, direction: GuardDirection::Down});

        let mut guard = Guard{x: 2, y: 2, direction: GuardDirection::Left};
        guard.step();
        assert_eq!(guard, Guard{x: 1, y: 2, direction: GuardDirection::Left});
    }

    #[test]
    fn test_is_guard_obstructed() {
        let obstructions = vec![
            Obstruction{x: 2, y: 1}
        ];
        let guard = Guard{x: 2, y: 2, direction: GuardDirection::Up};
        assert!(guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 3, y: 2}
        ];
        let guard = Guard{x: 2, y: 2, direction: GuardDirection::Right};
        assert!(guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 2, y: 3}
        ];
        let guard = Guard{x: 2, y: 2, direction: GuardDirection::Down};
        assert!(guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 1, y: 2}
        ];
        let guard = Guard{x: 2, y: 2, direction: GuardDirection::Left};
        assert!(guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 2, y: 2}
        ];
        let guard = Guard{x: 4, y: 4, direction: GuardDirection::Up};
        assert!(!guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 2, y: 2}
        ];
        let guard = Guard{x: 4, y: 4, direction: GuardDirection::Right};
        assert!(!guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 2, y: 2}
        ];
        let guard = Guard{x: 4, y: 4, direction: GuardDirection::Down};
        assert!(!guard.is_obstructed(&obstructions));

        let obstructions = vec![
            Obstruction{x: 2, y: 2}
        ];
        let guard = Guard{x: 4, y: 4, direction: GuardDirection::Left};
        assert!(!guard.is_obstructed(&obstructions));
    }
}
