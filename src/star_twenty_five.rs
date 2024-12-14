use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_five.txt").unwrap();
    let machines = parse_input(file.lines());

    println!("{:?}", solve_all_machines(&machines));

}

#[derive(PartialEq, Debug, Clone)]
struct MachineSettings {
    a_button: (isize, isize),
    b_button: (isize, isize),
    prize_location: (isize, isize)
}

fn parse_input<'a, I>(str_lines: I) -> Vec<MachineSettings>
where
    I: IntoIterator<Item = &'a str>
{
    let mut machines = vec![];
    let mut machine = MachineSettings {
        a_button: (0, 0),
        b_button: (0, 0),
        prize_location: (0, 0)
    };

    for line in str_lines {
        let line = String::from(line);
        if line.starts_with("Button A") {
            machine.a_button = get_xy(&line);
        } else if line.starts_with("Button B") {
            machine.b_button = get_xy(&line);
        } else if line.starts_with("Prize") {
            machine.prize_location = get_xy(&line);
            machines.push(machine.clone());
            machine = MachineSettings {
                a_button: (0, 0),
                b_button: (0, 0),
                prize_location: (0, 0)
            };
        }
    }

    machines
}


fn get_xy(string: &String) -> (isize, isize) {
    let mut split = string.split(",");
    let x_string = split.next().unwrap().chars().filter(|s| "0123456789".contains(*s)).collect::<String>().to_string();
    let y_string = split.next().unwrap().chars().filter(|s| "0123456789".contains(*s)).collect::<String>().to_string();
    (x_string.parse::<isize>().unwrap(), y_string.parse::<isize>().unwrap())
}

fn solve_all_machines(machines: &Vec<MachineSettings>) -> isize {
    machines.iter()
        .map(|machine| solve_machine(machine))
        .map(|solution| solution.unwrap_or(0))
        .sum()
}

// failed bruteforce attempt
// fn find_optimal_plays(x: isize, y: isize, machine: &MachineSettings, tokens_used: usize, a_presses: usize, b_presses: usize) -> (bool, usize) {
//     if a_presses > 100 || b_presses > 100 {
//         return (false, usize::MAX);
//     }
//     if x > machine.prize_location.0 || y > machine.prize_location.1 {
//         return (false, usize::MAX);
//     }
//     if x == machine.prize_location.0 && y == machine.prize_location.1 {
//         return (true, tokens_used)
//     }

//     if x < machine.prize_location.0 && y < machine.prize_location.1 {
//         // push B button
//         let (is_prize_found, found_tokens_used) = find_optimal_plays(
//             x + machine.b_button.0,
//             y + machine.b_button.1,
//             machine,
//             tokens_used + 1,
//             a_presses,
//             b_presses + 1,
//         );
//         if is_prize_found {
//             return (true, found_tokens_used);
//         }
//         // push A button
//         return find_optimal_plays(
//             x + machine.a_button.0,
//             y + machine.a_button.1,
//             machine,
//             tokens_used + 3,
//             a_presses + 1,
//             b_presses,
//         );
//     }

//     (false, usize::MAX)
// }

fn solve_machine(machine: &MachineSettings) -> Option<isize> {
    // Applying Cramer's rule

    let d = (machine.a_button.0 * machine.b_button.1) - (machine.a_button.1 * machine.b_button.0);
    if d == 0 {
        return None;
    }

    let dx = (machine.prize_location.0 * machine.b_button.1) - (machine.prize_location.1 * machine.b_button.0);
    let dy = (machine.prize_location.1 * machine.a_button.0) - (machine.prize_location.0 * machine.a_button.1);

    if dx % d != 0 || dy % d != 0 {
        return None;
    }

    let a = dx / d;
    let b = dy / d;

    if a < 0 || b < 0 || a > 100 || b > 100 {
        return None;
    }


    if (a * machine.a_button.0) + (b * machine.b_button.0) != machine.prize_location.0 ||
       (a * machine.a_button.1) + (b * machine.b_button.1) != machine.prize_location.1 {
        return None;
    }

    Some((a * 3) + (b * 1))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
            "",
            "Button A: X+26, Y+66",
            "Button B: X+67, Y+21",
            "Prize: X=12748, Y=12176",
            "",
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
            "",
            "Button A: X+69, Y+23",
            "Button B: X+27, Y+71",
            "Prize: X=18641, Y=10279",
        ].join("\n");
        let result = parse_input(input.lines());
        assert_eq!(
            result,
            [
                MachineSettings {
                    a_button: (94, 34),
                    b_button: (22, 67),
                    prize_location: (8400, 5400),
                },
                MachineSettings {
                    a_button: (26, 66),
                    b_button: (67, 21),
                    prize_location: (12748, 12176),
                },
                MachineSettings {
                    a_button: (17, 86),
                    b_button: (84, 37),
                    prize_location: (7870, 6450),
                },
                MachineSettings {
                    a_button: (69, 23),
                    b_button: (27, 71),
                    prize_location: (18641, 10279),
                },
            ],
        );

    }

    #[test]
    fn test_solve_machine() {
        let machine = MachineSettings {
            a_button: (94, 34),
            b_button: (22, 67),
            prize_location: (8400, 5400),
        };
        assert_eq!(solve_machine(&machine), Some(280));

        let machine = MachineSettings {
            a_button: (26, 66),
            b_button: (67, 21),
            prize_location: (12748, 12176),
        };
        assert_eq!(solve_machine(&machine), None);

        let machine = MachineSettings {
            a_button: (17, 86),
            b_button: (84, 37),
            prize_location: (7870, 6450),
        };
        assert_eq!(solve_machine(&machine), Some(200));

        let machine = MachineSettings {
            a_button: (69, 23),
            b_button: (27, 71),
            prize_location: (18641, 10279),
        };
        assert_eq!(solve_machine(&machine), None);
    }

    #[test]
    fn test_solve_all_machines() {
        let machines = vec![
            MachineSettings {
                a_button: (94, 34),
                b_button: (22, 67),
                prize_location: (8400, 5400),
            },
            MachineSettings {
                a_button: (26, 66),
                b_button: (67, 21),
                prize_location: (12748, 12176),
            },
            MachineSettings {
                a_button: (17, 86),
                b_button: (84, 37),
                prize_location: (7870, 6450),
            },
            MachineSettings {
                a_button: (69, 23),
                b_button: (27, 71),
                prize_location: (18641, 10279),
            },
        ];
        assert_eq!(solve_all_machines(&machines), 480);
    }
}

