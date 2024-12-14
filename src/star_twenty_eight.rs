use std::{collections::HashMap, fs};

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_eight.txt").unwrap();
    let mut robots = parse_input(file.lines());
    let w = 101; let h = 103;

    let mut iterations = 0;
    loop {
        println!("Loop #{}", iterations);
        print_robots(&robots, w, h);
        move_robots(&mut robots, w, h, 1);

        iterations += 1;
    }
}

fn parse_input<'a, I>(str_lines: I) -> Vec<Robot>
where
    I: IntoIterator<Item = &'a str>
{
    let mut robots = vec![];

    for line in str_lines {
        let mut line = line.split_whitespace();
        let p = line.next().unwrap().to_string();
        let v = line.next().unwrap().to_string();
        robots.push(Robot {
            position: get_xy(&p),
            velocity: get_xy(&v),
        })
    }

    robots
}

fn get_xy(string: &String) -> Point {
    let mut split = string.split(",");
    let x_string = split.next().unwrap().chars().filter(|s| "-0123456789".contains(*s)).collect::<String>().to_string();
    let y_string = split.next().unwrap().chars().filter(|s| "-0123456789".contains(*s)).collect::<String>().to_string();
    Point {
        x: x_string.parse::<isize>().unwrap(),
        y: y_string.parse::<isize>().unwrap(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point
}

impl Robot {
    pub fn do_move(&mut self, steps: isize, w: isize, h: isize) {
        self.position.x = (self.position.x + (self.velocity.x * steps)) % w;
        if self.position.x.is_negative() {
            self.position.x = (w + self.position.x) % w;
        }
        self.position.y = (self.position.y + (self.velocity.y * steps)) % h;
        if self.position.y.is_negative() {
            self.position.y = (h + self.position.y) % h;
        }
    }

}

fn move_robots(robots: &mut Vec<Robot>, w: isize, h: isize, steps: isize) {
    for robot in robots {
        robot.do_move(steps, w, h);
    }
}

fn print_robots(robots: &Vec<Robot>, w: isize, h: isize) {
    let mut unique_robot_locations = HashMap::new();
    for robot in robots {
        unique_robot_locations.insert((robot.position.x, robot.position.y), robot);
    }

    for y in 0..h {
        for x in 0..w {
            let char = match unique_robot_locations.get(&(x, y)) {
                Some(_) => 'X',
                None => ' ',
            };
            print!("{}", char);
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
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ].join("\n");
        let result = parse_input(input.lines());
        assert_eq!(
            result,
            [
                Robot {
                    position: Point {x: 0, y: 4},
                    velocity: Point {x: 3, y: -3},
                },
                Robot {
                    position: Point {x: 6, y: 3},
                    velocity: Point {x: -1, y: -3},
                },
                Robot {
                    position: Point {x: 10, y: 3},
                    velocity: Point {x: -1, y: 2},
                },
                Robot {
                    position: Point {x: 2, y: 0},
                    velocity: Point {x: 2, y: -1},
                },
                Robot {
                    position: Point {x: 0, y: 0},
                    velocity: Point {x: 1, y: 3},
                },
                Robot {
                    position: Point {x: 3, y: 0},
                    velocity: Point {x: -2, y: -2},
                },
                Robot {
                    position: Point {x: 7, y: 6},
                    velocity: Point {x: -1, y: -3},
                },
                Robot {
                    position: Point {x: 3, y: 0},
                    velocity: Point {x: -1, y: -2},
                },
                Robot {
                    position: Point {x: 9, y: 3},
                    velocity: Point {x: 2, y: 3},
                },
                Robot {
                    position: Point {x: 7, y: 3},
                    velocity: Point {x: -1, y: 2},
                },
                Robot {
                    position: Point {x: 2, y: 4},
                    velocity: Point {x: 2, y: -3},
                },
                Robot {
                    position: Point {x: 9, y: 5},
                    velocity: Point {x: -3, y: -3},
                },
            ],
        );

    }

    #[test]
    fn test_move_robot() {
        let mut robot = Robot {
            position: Point {x: 9, y: 5},
            velocity: Point {x: -3, y: -3},
        };
        robot.do_move(1, 11, 7);
        assert_eq!((robot.position.x, robot.position.y), (6, 2));

        let mut robot = Robot {
            position: Point {x: 2, y: 4},
            velocity: Point {x: 2, y: -3},
        };
        robot.do_move(5, 11, 7);
        assert_eq!((robot.position.x, robot.position.y), (1, 3));

        let mut robot = Robot {
            position: Point {x: 2, y: 2},
            velocity: Point {x: -3, y: -3},
        };
        robot.do_move(1, 11, 7);
        assert_eq!((robot.position.x, robot.position.y), (10, 6));
    }

}
