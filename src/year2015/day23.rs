//! Opening the Turing Lock
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    let mut com: Computer = input.into();
    com.compute()
}

pub fn part2(input: &str) -> usize {
    let mut com: Computer = input.into();
    com.a = 1;
    com.compute()
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

impl From<&str> for Register {
    fn from(s: &str) -> Self {
        match s {
            "a" | "a," => Self::A,
            "b" | "b," => Self::B,
            _ => panic!("Not implemented.s"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instructions {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpEven(Register, isize),
    JumpOne(Register, isize),
}

impl From<&str> for Instructions {
    fn from(s: &str) -> Self {
        let mut token = s.split(' ');
        let instruction = token.next().unwrap();
        match instruction {
            "jmp" => {
                let offset = token.next().unwrap().parse().unwrap();
                Instructions::Jump(offset)
            }
            "tpl" | "inc" | "hlf" => {
                let r: Register = token.next().unwrap().into();
                match instruction {
                    "inc" => Instructions::Increment(r),
                    "hlf" => Instructions::Half(r),
                    "tpl" => Instructions::Triple(r),
                    _ => panic!("Not implemented."),
                }
            }
            "jie" | "jio" => {
                let r: Register = token.next().unwrap().into();
                let offset = token.next().unwrap().parse().unwrap();
                match instruction {
                    "jie" => Instructions::JumpEven(r, offset),
                    "jio" => Instructions::JumpOne(r, offset),
                    _ => panic!("Not implemented."),
                }
            }
            _ => panic!("Not implemented."),
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    a: usize,
    b: usize,
    instruction_pointer: usize,
    instructions: Vec<Instructions>,
}

impl Computer {
    fn get_register(&mut self, r: Register) -> &mut usize {
        match r {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }

    fn register_is_even(&self, r: Register) -> bool {
        match r {
            Register::A => self.a.is_multiple_of(2),
            Register::B => self.b.is_multiple_of(2),
        }
    }

    fn register_is_one(&self, r: Register) -> bool {
        match r {
            Register::A => self.a == 1,
            Register::B => self.b == 1,
        }
    }

    fn jump(&mut self, offset: isize) {
        if offset > 0 {
            self.instruction_pointer += offset as usize;
        } else {
            self.instruction_pointer -= (-offset) as usize;
        }
    }

    fn compute(&mut self) -> usize {
        loop {
            if self.instruction_pointer >= self.instructions.len() {
                break;
            }

            match &self.instructions[self.instruction_pointer] {
                Instructions::Increment(reg) => {
                    *self.get_register(*reg) += 1;
                    self.instruction_pointer += 1;
                }
                Instructions::Half(reg) => {
                    *self.get_register(*reg) /= 2;
                    self.instruction_pointer += 1;
                }
                Instructions::Triple(reg) => {
                    *self.get_register(*reg) *= 3;
                    self.instruction_pointer += 1;
                }
                Instructions::Jump(offset) => {
                    self.jump(*offset);
                }
                Instructions::JumpEven(reg, offset) => {
                    if self.register_is_even(*reg) {
                        self.jump(*offset);
                    } else {
                        self.instruction_pointer += 1;
                    }
                }
                Instructions::JumpOne(reg, offset) => {
                    if self.register_is_one(*reg) {
                        self.jump(*offset);
                    } else {
                        self.instruction_pointer += 1;
                    }
                }
            }
        }

        *self.get_register(Register::B)
    }
}

impl From<&str> for Computer {
    fn from(s: &str) -> Self {
        let mut instructions: Vec<Instructions> = Vec::new();
        for line in s.lines() {
            instructions.push(line.into())
        }

        Computer { a: 0, b: 0, instruction_pointer: 0, instructions }
    }
}
