use std::fs;

pub fn run() {
    let file = fs::read_to_string("./inputs/star_thirty_four.txt").unwrap();
    let cpu = parse_input(file.lines());
    let result = fix_corrupted_program(&cpu);

    println!("Result: {}", result);
}

#[derive(PartialEq, Debug, Clone)]
struct Registers {
    a: isize,
    b: isize,
    c: isize,
    pc: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct CPU {
    registers: Registers,
    halted: bool,
    halt_if_different: bool,
    program: Vec<u8>,
    output: Vec<isize>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            halted: false,
            halt_if_different: false,
            program: vec![],
            output: vec![],
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                pc: 0,
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.tick();
        }
    }

    fn tick(&mut self) {
        if self.registers.pc >= self.program.len() {
            self.halted = true;
            return;
        }
        self.decode();
    }

    fn decode(&mut self) {
        let opcode = *self.program.get(self.registers.pc).unwrap_or(&0);
        match opcode {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => unreachable!(),
        }
    }

    fn parse_operand(&self, operand: u8, is_literal: bool) -> isize {
        if is_literal {
            return operand as isize;
        }
        match operand {
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => operand as isize,
        }
    }

    fn adv(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), false);
        self.registers.a = self.registers.a / 2_isize.pow(operand as u32);
        self.registers.pc += 2;
    }

    fn bxl(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), true);
        self.registers.b = self.registers.b ^ operand;
        self.registers.pc += 2;
    }

    fn bst(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), false);
        self.registers.b = operand & 0b111;
        self.registers.pc += 2;
    }

    fn jnz(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), true);
        if self.registers.a == 0 {
            self.registers.pc += 2;
            return
        }
        self.registers.pc = operand as usize & 0b111;
    }

    fn bxc(&mut self) {
        self.registers.b = self.registers.b ^ self.registers.c;
        self.registers.pc += 2;
    }

    fn out(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), false);
        let result = operand & 0b111;
        self.output.push(result);
        if self.halt_if_different {
            if result != self.program[self.output.len() - 1] as isize {
                self.halted = true;
            }
        }
        self.registers.pc += 2;
    }

    fn bdv(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), false);
        self.registers.b = self.registers.a / 2_isize.pow(operand as u32);
        self.registers.pc += 2;
    }

    fn cdv(&mut self) {
        let operand = self.parse_operand(*self.program.get(self.registers.pc + 1).unwrap_or(&0), false);
        self.registers.c = self.registers.a / 2_isize.pow(operand as u32);
        self.registers.pc += 2;
    }
}

fn parse_input<'a, I>(str_lines: I) -> CPU
where
    I: IntoIterator<Item = &'a str>
{
    let mut cpu = CPU::new();

    for str_line in str_lines {
        let l = str_line.to_string();
        if l.starts_with("Register A:") {
            cpu.registers.a = parse_register_data(&l);
        } else if l.starts_with("Register B:") {
            cpu.registers.b = parse_register_data(&l);
        } else if l.starts_with("Register C:") {
            cpu.registers.c = parse_register_data(&l);
        } else if l.starts_with("Program:") {
            cpu.program = parse_program(&l)
        }
    }

    cpu
}

fn parse_register_data(str_line: &String) -> isize {
    str_line[12..].to_string().trim().parse().unwrap()
}

fn parse_program(str_line: &String) -> Vec<u8> {
    str_line[8..].to_string()
        .trim()
        .split(",")
        .map(|b| b.trim().parse::<u8>().unwrap())
        .collect()
}

fn fix_corrupted_program(cpu: &CPU) -> isize {
    let mut current_a = 0;
    let mut cpu_copy = cpu.clone();
    cpu_copy.halt_if_different = false;
    loop {
        if current_a == cpu.registers.a {
            current_a += 1;
            continue;
        }
        cpu_copy.registers.a = current_a;
        cpu_copy.registers.b = cpu.registers.b;
        cpu_copy.registers.c = cpu.registers.c;
        cpu_copy.registers.pc = 0;
        cpu_copy.halted = false;
        cpu_copy.output = vec![];
        cpu_copy.run();
        if cpu_copy.program.len() != cpu_copy.output.len() {
            current_a += 1;
            continue;
        }
        let mut is_match = true;
        for (p, o) in cpu_copy.program.iter().zip(cpu_copy.output.iter()) {
            if *p as isize != *o {
                is_match = false;
                break;
            }
        }
        if !is_match {
            current_a += 1;
            continue;
        }
        return current_a;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = String::from("\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        );
        let cpu = parse_input(input.lines());
        let expected_cpu = CPU {
            halted: false,
            halt_if_different: false,
            program: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
            registers: Registers {
                a: 729,
                b: 0,
                c: 0,
                pc: 0,
            }
        };
        assert_eq!(cpu, expected_cpu);
    }

    #[test]
    fn test_run_program() {
        let input = String::from("\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        );
        let mut cpu = parse_input(input.lines());
        cpu.run();
        assert_eq!(cpu.output, [4,6,3,5,6,3,5,2,1,0]);

        let input = String::from("\
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
"
        );
        let mut cpu = parse_input(input.lines());
        cpu.run();
        assert_eq!(cpu.registers.b, 1);

        let input = String::from("\
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
"
        );
        let mut cpu = parse_input(input.lines());
        cpu.run();
        assert_eq!(cpu.output, [0,1,2]);

        let input = String::from("\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"
        );
        let mut cpu = parse_input(input.lines());
        cpu.run();
        assert_eq!(cpu.output, [4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(cpu.registers.a, 0);

        let input = String::from("\
Register A: 0
Register B: 29
Register C: 9

Program: 1,7
"
        );
        let mut cpu = parse_input(input.lines());
        cpu.run();
        // assert_eq!(cpu.registers.b, 27);

        let input = String::from("\
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
"
        );
        let mut cpu = parse_input(input.lines());
        cpu.run();
        assert_eq!(cpu.registers.b, 44354);
    }

    #[test]
    fn test_fix_corrupted_program() {
        let input = String::from("\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        );
        let cpu = parse_input(input.lines());
        let result = fix_corrupted_program(&cpu);
        assert_eq!(result, 117440);
    }
}
