use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_nine.txt").unwrap();
    let (mut robot, mut objects, movements) = parse_input(file.lines());
    make_all_movements(&mut robot, &mut objects, &movements);
    let result = sum_coords(&objects);

    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
}

#[derive(Debug, PartialEq, Clone)]
struct Object {
    objtype: char,
    position: Position,
}

impl Object {
    pub fn do_move(&mut self, movement: &Movement, map: &Vec<Object>) {
        if !self.can_move(movement, map) {
            return;
        }
        let v = movement.get_vector();
        let next_position = Position {
            x: self.position.x + v.x,
            y: self.position.y + v.y,
        };
        self.position = next_position;
    }

    pub fn move_to_empty_space(&mut self, movement: &Movement, map: &Vec<Object>) {
        let v = movement.get_vector();
        let mut next_position = Position {
            x: self.position.x + v.x,
            y: self.position.y + v.y,
        };
        loop {
            if let Some(_) = map.iter().find(|obj| obj.position == next_position) {
                let v = movement.get_vector();
                next_position = Position {
                    x: next_position.x + v.x,
                    y: next_position.y + v.y,
                };
            } else {
                break;
            }
        }
        self.position = next_position;
    }

    fn can_move(&self, movement: &Movement, map: &Vec<Object>) -> bool {
        let v = movement.get_vector();
        let next_position = Position {
            x: self.position.x + v.x,
            y: self.position.y + v.y,
        };
        if let Some(obj) = map.iter().find(|o| o.position == next_position) {
            if obj.objtype == '#' {
                return false;
            }
            if obj.objtype == 'O' {
                return obj.can_move(movement, map);
            }
        }

        return true;
    }
}

#[derive(Debug, PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

impl Movement {
    pub fn get_vector(&self) -> Position {
        match self {
            Movement::Up => Position{x: 0, y: -1},
            Movement::Down => Position{x: 0, y: 1},
            Movement::Left => Position{x: -1, y: 0},
            Movement::Right => Position{x: 1, y: 0},
        }
    }
}

fn parse_input<'a, I>(str_lines: I) -> (Object, Vec<Object>, Vec<Movement>)
where
    I: IntoIterator<Item = &'a str>
{
    let mut objects = vec![];
    let mut movements = vec![];
    let mut robot = Object{objtype: '@', position: Position {x: 0, y: 0}};

    let mut checking_map = true;

    let mut y = 0;
    for line in str_lines {
        if checking_map {
            let mut x = 0;
            for c in line.chars() {
                if "#O".contains(c) {
                    objects.push(Object {
                        objtype: c,
                        position: Position {x, y}
                    });
                } else if c == '@' {
                    robot.position.x = x;
                    robot.position.y = y;
                }
                x += 1;
            }
            y += 1;
        } else {
            for c in line.chars() {
                movements.push(match c {
                    '^' => Movement::Up,
                    'v' => Movement::Down,
                    '<' => Movement::Left,
                    '>' => Movement::Right,
                    _ => unreachable!(),
                });
            }
        }

        if line.trim().is_empty() {
            checking_map = false;
        }
    }

    (robot, objects, movements)
}

fn make_all_movements(robot: &mut Object, objects: &mut Vec<Object>, movements: &Vec<Movement>) {
    for movement in movements {
        robot.do_move(movement, objects);
        let obj_clone = objects.clone();
        if let Some(obj) = objects.iter_mut().find(|o| o.position == robot.position) {
            if obj.objtype == 'O' {
                obj.move_to_empty_space(movement, &obj_clone);
            }
        }
    }
}

fn sum_coords(objects: &Vec<Object>) -> isize {
    let mut result = 0;
    for obj in objects {
        if obj.objtype == 'O' {
            result += (100 * obj.position.y) + obj.position.x;
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "........",
            "#..O.O.#",
            "##@.O..#",
            "",
            "<^^>>>vv<v>>v<<"
        ].join("\n");
        let result = parse_input(input.lines());
        assert_eq!(
            result,
            (
                Object{objtype: '@', position: Position{x: 2, y: 2}},
                vec![
                    Object{objtype: '#', position: Position{x: 0, y: 1}},
                    Object{objtype: 'O', position: Position{x: 3, y: 1}},
                    Object{objtype: 'O', position: Position{x: 5, y: 1}},
                    Object{objtype: '#', position: Position{x: 7, y: 1}},
                    Object{objtype: '#', position: Position{x: 0, y: 2}},
                    Object{objtype: '#', position: Position{x: 1, y: 2}},
                    Object{objtype: 'O', position: Position{x: 4, y: 2}},
                    Object{objtype: '#', position: Position{x: 7, y: 2}},
                ],
                vec![
                    Movement::Left,
                    Movement::Up,
                    Movement::Up,
                    Movement::Right,
                    Movement::Right,
                    Movement::Right,
                    Movement::Down,
                    Movement::Down,
                    Movement::Left,
                    Movement::Down,
                    Movement::Right,
                    Movement::Right,
                    Movement::Down,
                    Movement::Left,
                    Movement::Left,
                ]
            )
        )
    }

    #[test]
    fn test_robot_can_move() {
        let input = vec![
            "########",
            "#..O.O.#",
            "##@.O..#",
            "#...O..#",
            "#.#.O..#",
            "#...O..#",
            "#......#",
            "########",
            "",
            "<^^>>>vv<v>>v<<",
        ].join("\n");
        let (mut robot, objects, _) = parse_input(input.lines());

        assert!(!robot.can_move(&Movement::Left, &objects));

        robot.position.x = 3;
        robot.position.y = 2;
        assert!(!robot.can_move(&Movement::Up, &objects));

        robot.position.x = 4;
        robot.position.y = 1;
        assert!(robot.can_move(&Movement::Down, &objects));
    }

    #[test]
    fn test_sum_coords() {
        let input = vec![
            "#######",
            "#...O..",
            "#......",
            "",
            "<^^>>>vv<v>>v<<",
        ].join("\n");
        let (_, objects, _) = parse_input(input.lines());
        assert_eq!(sum_coords(&objects), 104);

        let input = vec![
            "########",
            "#..O.O.#",
            "##@.O..#",
            "#...O..#",
            "#.#.O..#",
            "#...O..#",
            "#......#",
            "########",
            "",
            "<^^>>>vv<v>>v<<",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(sum_coords(&objects), 2028);

        let input = vec![
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#..O@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
            "",
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(sum_coords(&objects), 10092);
    }
}

