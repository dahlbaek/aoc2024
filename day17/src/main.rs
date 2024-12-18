use std::str::FromStr;

const PUZZLE: &str = include_str!("puzzle");

struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn new(a: u64) -> Registers {
        Registers { a, b: 0, c: 0 }
    }
}

impl Registers {
    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone)]
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

type Operand = u64;

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

fn run(mut registers: Registers, program: &Program) -> String {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let (instruction, operand) = program[instruction_pointer];
        match instruction {
            Instruction::Adv => registers.a >>= registers.combo(operand),
            Instruction::Bxl => registers.b ^= operand,
            Instruction::Bst => registers.b = registers.combo(operand) % 8,
            Instruction::Jnz => {
                if registers.a != 0 {
                    instruction_pointer = operand.try_into().unwrap();
                    continue;
                }
            }
            Instruction::Bxc => registers.b ^= registers.c,
            Instruction::Out => output.push((registers.combo(operand) % 8).to_string()),
            Instruction::Bdv => registers.b = registers.a >> registers.combo(operand),
            Instruction::Cdv => registers.c = registers.a >> registers.combo(operand),
        }
        instruction_pointer += 1;
    }
    output.join(",")
}

fn get_output_end(n: usize) -> &'static str {
    let s = PUZZLE.trim().lines().last().unwrap();
    &s[s.len() + 1 - 2 * n..]
}

fn main() {
    let (registers, program) = parse();
    println!("Part 1: {}", run(registers, &program));

    // Program has the form
    //
    // while a != 0:
    //     # bitwise stuff
    //     a = a >> 3
    //
    // so we can compute the input value of a by starting with
    // an a that only has the lowest 3 bits set, then shifting
    // those 3 to the left and figuring out the next value of
    // the lowest 3 bits.
    let (_, program) = parse();
    let mut stack = vec![0u64];
    for j in 1..=program.len() * 2 {
        let expected = get_output_end(j);
        let mut new_stack = Vec::new();
        for a in stack.iter() {
            for lower in 0..8 {
                let current_a = (a << 3) + lower;
                if run(Registers::new(current_a), &program) == expected {
                    new_stack.push(current_a);
                }
            }
        }
        stack = new_stack
    }

    println!("Part 2: {}", stack.into_iter().min().unwrap())
}
