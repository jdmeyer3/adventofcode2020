use std::borrow::{Borrow, BorrowMut};

#[derive(Debug)]
enum Ops {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

impl Ops {
    fn execute(&self, state: &mut State) {
        match self {
            Ops::NOP(_) => {
                state.cursor += 1;
            }
            Ops::ACC(val) => {
                state.accumulator += *val;
                state.cursor += 1;
            }
            Ops::JMP(val) => {
                println!("jump {:?}", val);
                state.cursor += *val;
            }
        }
    }
}

#[derive(Default, Debug)]
struct State {
    accumulator: isize,
    cursor: isize,
}

#[derive(Debug)]
struct Instructions {
    visited: bool,
    op: Ops,
}

#[derive(Default, Debug)]
struct InstructionSet {
    instructions: Vec<Instructions>,
    state: State,
}

impl InstructionSet {

    fn execute(&mut self) -> isize {
        println!("{:?}", self.state);
        let instr = self.instructions[self.state.cursor as usize].borrow_mut();
        println!("next instructions {:?}", instr);
        if instr.visited {
            return self.state.accumulator;
        }
        let state = &mut self.state;
        instr.op.execute(state);
        instr.visited = true;
        self.execute()
    }
}

fn main() {
    let val = -4;

    let input = std::fs::read_to_string("input/day8").unwrap();
    let mut instructions = InstructionSet::default();
    for l in input.lines() {
        let mut instr = l.split(' ');
        match instr.next().unwrap() {
            "acc" => {
                instructions.instructions.push(Instructions {
                    visited: false,
                    op: Ops::ACC(instr.next().unwrap().parse().unwrap()),
                })
            }
            "jmp" => {
                instructions.instructions.push(Instructions {
                    visited: false,
                    op: Ops::JMP(instr.next().unwrap().parse().unwrap()),
                })
            }
            "nop" => {
                instructions.instructions.push(Instructions {
                    visited: false,
                    op: Ops::NOP(instr.next().unwrap().parse().unwrap()),
                })
            }
            &_ => {}
        }
    }
    let foo = instructions.execute();
    println!("pt1. {:?}", foo)
}
