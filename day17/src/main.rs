use std::str::FromStr;

const PUZZLE: &str = include_str!("puzzle");

struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

impl Registers {
    fn combo(&self, &operand: &u32) -> u32 {
        let raw = match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        };
        raw.into()
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Instruction, String> {
        match s {
            "0" => Ok(Instruction::Adv),
            "1" => Ok(Instruction::Bxl),
            "2" => Ok(Instruction::Bst),
            "3" => Ok(Instruction::Jnz),
            "4" => Ok(Instruction::Bxc),
            "5" => Ok(Instruction::Out),
            "6" => Ok(Instruction::Bdv),
            "7" => Ok(Instruction::Cdv),
            _ => Err(s.to_string()),
        }
    }
}

type Operand = u32;

type Program = Vec<(Instruction, Operand)>;

fn parse() -> (Registers, Program) {
    let mut lines = PUZZLE.trim().lines();
    let a = lines.next().unwrap()[12..].parse().unwrap();
    let b = lines.next().unwrap()[12..].parse().unwrap();
    let c = lines.next().unwrap()[12..].parse().unwrap();
    assert_eq!(lines.next().unwrap(), "");
    let program = lines.next().unwrap()[9..]
        .split(',')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            (
                Instruction::from_str(chunk[0]).unwrap(),
                chunk[1].parse().unwrap(),
            )
        })
        .collect();
    (Registers { a, b, c }, program)
}

fn run(mut registers: Registers, program: Program) -> String {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let (instruction, operand) = &program[instruction_pointer];
        match instruction {
            Instruction::Adv => registers.a = registers.a / 2u32.pow(registers.combo(operand)),
            Instruction::Bxl => registers.b = registers.b ^ operand,
            Instruction::Bst => registers.b = registers.combo(operand) % 8,
            Instruction::Jnz => {
                if registers.a != 0 {
                    instruction_pointer = (*operand).try_into().unwrap();
                    continue;
                }
            }
            Instruction::Bxc => registers.b = registers.b ^ registers.c,
            Instruction::Out => output.push((registers.combo(operand) % 8).to_string()),
            Instruction::Bdv => registers.b = registers.a / 2u32.pow(registers.combo(operand)),
            Instruction::Cdv => registers.c = registers.a / 2u32.pow(registers.combo(operand)),
        }
        instruction_pointer += 1;
    }
    output.join(",")
}

fn main() {
    let (registers, program) = parse();
    println!("Part 1: {}", run(registers, program));
}
