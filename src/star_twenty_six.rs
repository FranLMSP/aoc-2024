use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_twenty_six.txt").unwrap();
    let machines = parse_input(file.lines());

    println!("{:?}", solve_all_machines(&machines));

}

#[derive(PartialEq, Debug, Clone)]
struct MachineSettings {
    a_button: (i128, i128),
    b_button: (i128, i128),
    prize_location: (i128, i128)
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
            machine.a_button = get_xy(&line, false);
        } else if line.starts_with("Button B") {
            machine.b_button = get_xy(&line, false);
        } else if line.starts_with("Prize") {
            machine.prize_location = get_xy(&line, true);
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


fn get_xy(string: &String, is_prize: bool) -> (i128, i128) {
    let mut split = string.split(",");
    let prefix = match is_prize {
        true => 10000000000000,
        false => 0,
    };
    let x_string = split.next().unwrap().chars().filter(|s| "0123456789".contains(*s)).collect::<String>().to_string();
    let y_string = split.next().unwrap().chars().filter(|s| "0123456789".contains(*s)).collect::<String>().to_string();
    (x_string.parse::<i128>().unwrap() + prefix, y_string.parse::<i128>().unwrap() + prefix)
}

fn solve_all_machines(machines: &Vec<MachineSettings>) -> i128 {
    machines.iter()
        .map(|machine| solve_machine(machine))
        .map(|solution| solution.unwrap_or(0))
        .sum()
}

fn solve_machine(machine: &MachineSettings) -> Option<i128> {
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

    if a < 0 || b < 0 {
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
                    prize_location: (10000000008400, 10000000005400),
                },
                MachineSettings {
                    a_button: (26, 66),
                    b_button: (67, 21),
                    prize_location: (10000000012748, 10000000012176),
                },
                MachineSettings {
                    a_button: (17, 86),
                    b_button: (84, 37),
                    prize_location: (10000000007870, 10000000006450),
                },
                MachineSettings {
                    a_button: (69, 23),
                    b_button: (27, 71),
                    prize_location: (10000000018641, 10000000010279),
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

