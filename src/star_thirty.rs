use std::{collections::HashMap, fs};

pub fn run() {
    let file = fs::read_to_string("./inputs/star_thirty.txt").unwrap();
    let (mut robot, mut objects, movements) = parse_input(file.lines());
    make_all_movements(&mut robot, &mut objects, &movements);
    let result = sum_coords(&objects);

    // 1452076 too low
    // 1452348
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
}

impl Position {
}

#[derive(Debug, PartialEq, Clone)]
struct Object {
    id: String,
    objtype: char,
    position: Position,
}

impl Object {
    // for robot
    pub fn do_move(&mut self, movement: &Movement) {
        let v = movement.get_vector();
        let next_position = Position {
            x: self.position.x + v.x,
            y: self.position.y + v.y,
            w: self.position.w,
            h: self.position.h,
        };
        self.position = next_position;
    }

    fn can_move(&self, movement: &Movement, map: &Vec<Object>, found_static_objects: &mut Vec<Object>) -> bool {
        let v = movement.get_vector();
        let next_position = Position {
            // x: self.position.x + (v.x * self.position.w),
            // y: self.position.y + (v.y * self.position.h),
            x: self.position.x + v.x,
            y: self.position.y + v.y,
            w: self.position.w,
            h: self.position.h,
        };
        let filter_found_objects = map.iter()
            .filter(|o| movement_condition(&self.position, &o.position, movement) && o.id != self.id && is_hitting_hitbox(&o.position, &next_position));
        for found_object in filter_found_objects {
            if found_object.objtype == '#' {
                found_static_objects.push(found_object.clone());
                return false;
            }
            if found_object.objtype == 'O' {
                found_object.can_move(movement, map, found_static_objects);
            }
        }

        return found_static_objects.is_empty();
    }

    fn find_pushable_objects(&self, movement: &Movement, map: &Vec<Object>, pushable_objects: &mut Vec<Object>) {
        let v = movement.get_vector();
        let next_position = Position {
            x: self.position.x + v.x,
            y: self.position.y + v.y,
            w: self.position.w,
            h: self.position.h,
        };
        let filter_pushable_objects = map.iter()
            .filter(|o| o.id != self.id && o.objtype == 'O' && is_hitting_hitbox(&o.position, &next_position));
        for obj in filter_pushable_objects {
            let existing_obj = pushable_objects.iter().find(|o| o.id == obj.id);
            if existing_obj.is_none() {
                let new_obj = obj.clone();
                pushable_objects.push(new_obj.clone());
                new_obj.find_pushable_objects(movement, map, pushable_objects);
            }
        }
    }
}

fn is_hitting_hitbox(pos_a: &Position, pos_b: &Position) -> bool {
    if pos_a.x < 0 || pos_b.x < 0 || pos_a.y < 0 || pos_b.y < 0 {
        return true;
    }
    ((pos_b.x >= pos_a.x) && (pos_b.x <= pos_a.x + pos_a.w-1)
        && (pos_b.y >= pos_a.y) && (pos_b.y <= pos_a.y + pos_a.h-1))
        || ((pos_b.x + pos_b.w-1 >= pos_a.x) && (pos_b.x + pos_b.w-1 <= pos_a.x + pos_a.w-1)
        && (pos_b.y + pos_b.h-1 >= pos_a.y) && (pos_b.y + pos_b.h-1 <= pos_a.y + pos_a.h-1))
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
            Movement::Up =>    Position{x: 0,  y: -1, w: 0, h: 0},
            Movement::Down =>  Position{x: 0,  y: 1,  w: 0, h: 0},
            Movement::Left =>  Position{x: -1, y: 0,  w: 0, h: 0},
            Movement::Right => Position{x: 1,  y: 0,  w: 0, h: 0},
        }
    }
}

fn parse_input<'a, I>(str_lines: I) -> (Object, Vec<Object>, Vec<Movement>)
where
    I: IntoIterator<Item = &'a str>
{
    let mut objects = vec![];
    let mut movements = vec![];
    let mut robot = Object{id: String::new(), objtype: '@', position: Position {x: 0, y: 0, w: 1, h: 1}};

    let mut checking_map = true;

    let mut y = 0;
    for line in str_lines {
        if checking_map {
            let mut x = 0;
            for c in line.chars() {
                if "#O".contains(c) {
                    objects.push(Object {
                        id: format!("{}{}-{}-{}-{}", c, x, y, 2, 1),
                        objtype: c,
                        position: Position {x: x, y: y, w: 2, h: 1}
                    });
                } else if c == '@' {
                    robot.id = format!("{}{}-{}-{}-{}", c, x, y, 1, 1);
                    robot.position.x = x;
                    robot.position.y = y;
                }
                x += 2;
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
        print_board(robot, objects, 20, 10);
        // println!("{:?}", movement);

        if !robot.can_move(movement, objects, &mut vec![]) {
            // println!("CANNOT MOVE");
            continue;
        }
        let mut pushable_objects = vec![];
        robot.find_pushable_objects(movement, objects, &mut pushable_objects);
        robot.do_move(movement);
        for obj in pushable_objects {
            if let Some(obj) = objects.iter_mut()
                .find(|o| o.id == obj.id)
            {
                obj.do_move(movement);
            }
        }
    }
    print_board(robot, objects, 20, 10);
}

fn movement_condition(current_pos: &Position, obj_position: &Position, movement: &Movement) -> bool {
    match movement {
        Movement::Up => obj_position.y <= current_pos.y,
        Movement::Down => obj_position.y >= current_pos.y,
        Movement::Left => obj_position.x <= current_pos.x,
        Movement::Right => obj_position.x >= current_pos.x,
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

fn print_board(robot: &Object, objects: &Vec<Object>, w: usize, h: usize) {
    let mut map = HashMap::new();
    for o in objects {
        map.insert((o.position.x, o.position.y), o);
    }
    for y in 0..h {
        let mut x = 0;
        while x < w {
            if robot.position.x == x as isize && robot.position.y == y as isize {
                print!("@");
                x += 1;
                continue;
            }

            let (c, obj_w) = match map.get(&(x as isize, y as isize)) {
                Some(o) => (o.objtype, o.position.w),
                None => ('.', 1)
            };
            for _ in 0..obj_w {
                print!("{}", c);
                x += 1;
            }
        }
        println!("");
    }
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
                Object{id: format!("@4-2-1-1"), objtype: '@', position: Position{x: 4, y: 2, w: 1, h: 1}},
                vec![
                    Object{id: format!("#0-1-2-1"),  objtype: '#', position: Position{x: 0, y: 1, w: 2, h: 1}},
                    Object{id: format!("O6-1-2-1"),  objtype: 'O', position: Position{x: 6, y: 1, w: 2, h: 1}},
                    Object{id: format!("O10-1-2-1"), objtype: 'O', position: Position{x:10, y: 1, w: 2, h: 1}},
                    Object{id: format!("#14-1-2-1"), objtype: '#', position: Position{x:14, y: 1, w: 2, h: 1}},
                    Object{id: format!("#0-2-2-1"),  objtype: '#', position: Position{x: 0, y: 2, w: 2, h: 1}},
                    Object{id: format!("#2-2-2-1"),  objtype: '#', position: Position{x: 2, y: 2, w: 2, h: 1}},
                    Object{id: format!("O8-2-2-1"),  objtype: 'O', position: Position{x: 8, y: 2, w: 2, h: 1}},
                    Object{id: format!("#14-2-2-1"), objtype: '#', position: Position{x:14, y: 2, w: 2, h: 1}},
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
            "........",
            "....@...",
            "....O...",
            "....O...",
            "....O...",
            "....O...",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (robot, objects, _) = parse_input(input.lines());
        assert!(robot.can_move(&Movement::Down, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            "...@O...",
            "....O...",
            "....O...",
            "....O...",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (mut robot, objects, _) = parse_input(input.lines());
        robot.position.x = 7;
        robot.position.y = 2;
        assert!(robot.can_move(&Movement::Right, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            "...@#...",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (mut robot, objects, _) = parse_input(input.lines());
        robot.position.x = 7;
        robot.position.y = 2;
        assert!(!robot.can_move(&Movement::Right, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            "...@OO#.",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (mut robot, objects, _) = parse_input(input.lines());
        robot.position.x = 7;
        robot.position.y = 2;
        assert!(!robot.can_move(&Movement::Right, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            "...#OO@.",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (robot, objects, _) = parse_input(input.lines());
        assert!(!robot.can_move(&Movement::Left, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            ".#OOOO@.",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (robot, objects, _) = parse_input(input.lines());
        assert!(!robot.can_move(&Movement::Left, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            ".@OOOO..",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (robot, objects, _) = parse_input(input.lines());
        assert!(robot.can_move(&Movement::Right, &objects, &mut vec![]));

        let input = vec![
            "........",
            "........",
            ".@OOO.O#",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>",
        ].join("\n");
        let (mut robot, objects, _) = parse_input(input.lines());
        robot.do_move(&Movement::Right);
        assert!(robot.can_move(&Movement::Right, &objects, &mut vec![]));
    }

    #[test]
    fn test_move() {
        let input = vec![
            "........",
            "......@.",
            ".....O..",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            ">",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O10-2-2-1"),  objtype: 'O', position: Position{x: 10, y: 2, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "........",
            "....OO@.",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            "<",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O8-2-2-1"),  objtype: 'O', position: Position{x: 7, y: 2, w: 2, h: 1}},
            Object{id: format!("O10-2-2-1"), objtype: 'O', position: Position{x: 9, y: 2, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "........",
            ".....O@.",
            "....O...",
            "........",
            "........",
            "........",
            "........",
            "",
            "<^<v",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O10-2-2-1"), objtype: 'O', position: Position{x: 9, y: 3, w: 2, h: 1}},
            Object{id: format!("O8-3-2-1"),  objtype: 'O', position: Position{x: 8, y: 4, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "........",
            "..@O....",
            "....O...",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>^>v",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 3, w: 2, h: 1}},
            Object{id: format!("O8-3-2-1"),  objtype: 'O', position: Position{x: 8, y: 4, w: 2, h: 1}},
        ]);

        // let input = vec![
        //     "........",
        //     "........",
        //     "..@O....",
        //     "....O...",
        //     "...#....",
        //     "........",
        //     "........",
        //     "........",
        //     "",
        //     ">>^>v",
        // ].join("\n");
        // let (mut robot, mut objects, movements) = parse_input(input.lines());
        // make_all_movements(&mut robot, &mut objects, &movements);
        // assert_eq!(objects, [
        //     Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 3, w: 2, h: 1}},
        //     Object{id: format!("O8-3-2-1"),  objtype: 'O', position: Position{x: 8, y: 4, w: 2, h: 1}},
        // ]);

        let input = vec![
            "........",
            "........",
            "..@O....",
            "....O...",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>^>v",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 3, w: 2, h: 1}},
            Object{id: format!("O8-3-2-1"),  objtype: 'O', position: Position{x: 8, y: 4, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "........",
            "..@OO...",
            "....O...",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>v>^",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 1, w: 2, h: 1}},
            Object{id: format!("O8-2-2-1"), objtype: 'O', position: Position{x: 9, y: 2, w: 2, h: 1}},
            Object{id: format!("O8-3-2-1"),  objtype: 'O', position: Position{x: 8, y: 3, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "....#...",
            "..@O....",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>v>^",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("#8-1-2-1"), objtype: '#', position: Position{x: 8, y: 1, w: 2, h: 1}},
            Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 2, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "...OO...",
            "..@O....",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>v>^",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O6-1-2-1"), objtype: 'O', position: Position{x: 6, y: 0, w: 2, h: 1}},
            Object{id: format!("O8-1-2-1"), objtype: 'O', position: Position{x: 8, y: 0, w: 2, h: 1}},
            Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 1, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "...O....",
            "...O....",
            "...OO...",
            "..@O....",
            "........",
            "........",
            "........",
            "",
            ">^^><vv>v>^",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O6-1-2-1"), objtype: 'O', position: Position{x: 6, y: 0, w: 2, h: 1}},
            Object{id: format!("O6-2-2-1"), objtype: 'O', position: Position{x: 7, y: 1, w: 2, h: 1}},
            Object{id: format!("O6-3-2-1"), objtype: 'O', position: Position{x: 6, y: 2, w: 2, h: 1}},
            Object{id: format!("O8-3-2-1"), objtype: 'O', position: Position{x: 8, y: 2, w: 2, h: 1}},
            Object{id: format!("O6-4-2-1"), objtype: 'O', position: Position{x: 7, y: 3, w: 2, h: 1}},
        ]);

        let mut objects = vec![
            Object{id: format!("1"), objtype: 'O', position: Position{x: 7, y: 0, w: 2, h: 1}},
            Object{id: format!("2"), objtype: 'O', position: Position{x: 9, y: 0, w: 2, h: 1}},
            Object{id: format!("3"), objtype: 'O', position: Position{x: 12, y: 0, w: 2, h: 1}},
            Object{id: format!("4"), objtype: 'O', position: Position{x: 16, y: 0, w: 2, h: 1}},
        ];
        let mut robot = Object{id: format!("0"), objtype: '@', position: Position{x: 6, y: 0, w: 1, h: 1}};
        let movements = vec![Movement::Right];
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("1"), objtype: 'O', position: Position{x: 8, y: 0, w: 2, h: 1}},
            Object{id: format!("2"), objtype: 'O', position: Position{x: 10, y: 0, w: 2, h: 1}},
            Object{id: format!("3"), objtype: 'O', position: Position{x: 12, y: 0, w: 2, h: 1}},
            Object{id: format!("4"), objtype: 'O', position: Position{x: 16, y: 0, w: 2, h: 1}},
        ]);
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("1"), objtype: 'O', position: Position{x: 9, y: 0, w: 2, h: 1}},
            Object{id: format!("2"), objtype: 'O', position: Position{x: 11, y: 0, w: 2, h: 1}},
            Object{id: format!("3"), objtype: 'O', position: Position{x: 13, y: 0, w: 2, h: 1}},
            Object{id: format!("4"), objtype: 'O', position: Position{x: 16, y: 0, w: 2, h: 1}},
        ]);
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("1"), objtype: 'O', position: Position{x: 10, y: 0, w: 2, h: 1}},
            Object{id: format!("2"), objtype: 'O', position: Position{x: 12, y: 0, w: 2, h: 1}},
            Object{id: format!("3"), objtype: 'O', position: Position{x: 14, y: 0, w: 2, h: 1}},
            Object{id: format!("4"), objtype: 'O', position: Position{x: 16, y: 0, w: 2, h: 1}},
        ]);
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("1"), objtype: 'O', position: Position{x: 11, y: 0, w: 2, h: 1}},
            Object{id: format!("2"), objtype: 'O', position: Position{x: 13, y: 0, w: 2, h: 1}},
            Object{id: format!("3"), objtype: 'O', position: Position{x: 15, y: 0, w: 2, h: 1}},
            Object{id: format!("4"), objtype: 'O', position: Position{x: 17, y: 0, w: 2, h: 1}},
        ]);

        let input = vec![
            "........",
            "........",
            "..@OO.O#",
            "........",
            "........",
            "........",
            "........",
            "........",
            "",
            ">>>",
        ].join("\n");
        let (mut robot, mut objects, movements) = parse_input(input.lines());
        make_all_movements(&mut robot, &mut objects, &movements);
        assert_eq!(objects, [
            Object{id: format!("O6-2-2-1"),  objtype: 'O', position: Position{x: 8, y: 2, w: 2, h: 1}},
            Object{id: format!("O8-2-2-1"),  objtype: 'O', position: Position{x: 10, y: 2, w: 2, h: 1}},
            Object{id: format!("O12-2-2-1"),  objtype: 'O', position: Position{x: 12, y: 2, w: 2, h: 1}},
            Object{id: format!("#14-2-2-1"),  objtype: '#', position: Position{x: 14, y: 2, w: 2, h: 1}},
        ]);
    }

    #[test]
    fn test_sum_coords() {
        // let input = vec![
        //     "#######",
        //     "#...#.#",
        //     "#.....#",
        //     "#..OO@#",
        //     "#..O..#",
        //     "#.....#",
        //     "#######",
        //     "",
        //     "<vv<<^^<<^^",
        // ].join("\n");
        // let (mut robot, mut objects, movements) = parse_input(input.lines());
        // make_all_movements(&mut robot, &mut objects, &movements);
        // print_board(&robot, &objects, 14, 7);
        // assert_eq!(sum_coords(&objects), 1410);

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
        // print_board(&robot, &objects, 20, 10);
        assert_eq!(sum_coords(&objects), 9021);
    }
}

