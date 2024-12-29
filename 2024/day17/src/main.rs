use std::collections::HashSet;
use std::env;
use std::fs;

struct StateMachine {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

impl StateMachine {
    #[allow(dead_code)]
    fn print_state(&self) {
        println!("Register A: {}", self.a);
        println!("Register B: {}", self.b);
        println!("Register C: {}", self.c);
        println!("Output: {:?}", self.output);
    }

    fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    fn read_instruction(&mut self) -> Option<i64> {
        if self.pc >= self.program.len() {
            None
        } else {
            let ret = self.program[self.pc];
            self.pc += 1;
            Some(ret)
        }
    }

    fn combo_operand(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            c => {
                panic!("Invalid combo operand {}", c)
            }
        }
    }

    fn literal_operand(&self, operand: i64) -> i64 {
        operand
    }

    fn handle_adv(&mut self, operand: i64) {
        // Performs division
        let numerator = self.a;
        let denominator = 2_i64.pow(self.combo_operand(operand) as u32);
        self.a = numerator / denominator;
    }

    fn handle_bxl(&mut self, operand: i64) {
        self.b = self.b ^ self.literal_operand(operand);
    }

    fn handle_bst(&mut self, operand: i64) {
        self.b = self.combo_operand(operand) % 8;
    }

    fn handle_jnz(&mut self, operand: i64) {
        if self.a != 0 {
            self.set_pc(self.literal_operand(operand) as usize);
        }
    }

    fn handle_bxc(&mut self, _operand: i64) {
        self.b = self.b ^ self.c;
    }

    fn handle_out(&mut self, operand: i64) {
        self.output.push(self.combo_operand(operand) % 8);
    }

    fn handle_bdv(&mut self, operand: i64) {
        let numerator = self.a;
        let denominator = 2_i64.pow(self.combo_operand(operand) as u32);
        self.b = numerator / denominator;
    }

    fn handle_cdv(&mut self, operand: i64) {
        let numerator = self.a;
        let denominator = 2_i64.pow(self.combo_operand(operand) as u32);
        self.c = numerator / denominator;
    }

    fn do_instruction(&mut self) -> Option<()> {
        let opcode = self.read_instruction();
        let operand = self.read_instruction();

        if opcode.is_none() || operand.is_none() {
            return None;
        }

        let opcode = opcode.unwrap();
        let operand = operand.unwrap();

        match opcode {
            0 => {
                self.handle_adv(operand);
            }
            1 => {
                self.handle_bxl(operand);
            }
            2 => {
                self.handle_bst(operand);
            }
            3 => {
                self.handle_jnz(operand);
            }
            4 => {
                self.handle_bxc(operand);
            }
            5 => {
                self.handle_out(operand);
            }
            6 => {
                self.handle_bdv(operand);
            }
            7 => {
                self.handle_cdv(operand);
            }
            c => {
                panic!("Invalid opcode {}", c);
            }
        }

        Some(())
    }
}

fn parse_program(p: &str) -> Vec<i64> {
    let p: Vec<&str> = p.trim().split(": ").collect();
    p[1].split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_registers(input: Vec<&str>) -> (i64, i64, i64) {
    let a: i64 = input[0].split(": ").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();
    let b: i64 = input[1].split(": ").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();
    let c: i64 = input[2].split(": ").collect::<Vec<&str>>()[1]
        .trim()
        .parse()
        .unwrap();
    (a, b, c)
}

fn check_nth_output(mut machine: StateMachine, candidate: &str, n: usize) -> bool {
    let a = i64::from_str_radix(candidate, 2).unwrap();
    machine.a = a;
    loop {
        let res = machine.do_instruction();
        if res.is_none() {
            break;
        }
    }
    machine.output[n] == machine.program[n]
}

fn solve(input: &str) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let registers: Vec<&str> = input[0].lines().collect();
    let program = parse_program(input[1]);
    let registers = parse_registers(registers);

    let mut machine = StateMachine {
        a: registers.0,
        b: registers.1,
        c: registers.2,
        pc: 0,
        program: program.clone(),
        output: Vec::new(),
    };

    let mut candidate: i64 = 8_i64.pow(15);

    let mut tmp = candidate;

    let mut possible_outputs = HashSet::new();

    loop {
        machine.a = candidate;
        machine.b = registers.1;
        machine.c = registers.2;
        machine.pc = 0;
        machine.output.clear();

        loop {
            let res = machine.do_instruction();
            if res.is_none() {
                break;
            }
        }

        possible_outputs.insert(machine.output.clone());

        if machine.output[0] == 2
            && machine.output[1] == 4
            && machine.output[2] == 1
            && machine.output[3] == 1
            && machine.output[4] == 7
            && machine.output[5] == 5
        {
            println!("===========");
            println!("candidate: {}", candidate);
            println!("candidate binary: {:b}", candidate);
            println!("diff: {}", candidate - tmp);
            tmp = candidate;
            machine.print_state();
            println!("possible outputs size {}", possible_outputs.len());
        }

        if machine.output == program {
            break;
        }

        candidate += 1;
    }

    println!("ans {}", candidate);
}

fn main() {
    let path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let input = fs::read_to_string(path).expect("File should exist");
    solve(&input);
}
