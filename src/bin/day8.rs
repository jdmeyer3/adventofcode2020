use std::borrow::{BorrowMut};

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
                state.cursor += *val;
            }
        }
    }
}

#[derive(Default, Debug)]
struct State {
    accumulator: isize,
    cursor: isize,

    // did the instruction set finish
    complete: bool,
}

#[derive(Debug)]
struct Instructions {
    visited: bool,
    op: Ops,
}

impl Instructions {
    fn flip(&mut self) {
        match self.op {
            Ops::NOP(val) => {
                self.op = Ops::JMP(val)
            }
            Ops::ACC(_) => {
                panic!("should not be flipped")
            }
            Ops::JMP(val) => {
                self.op = Ops::NOP(val)
            }
        }
    }
    fn reset(&mut self) {
        self.visited = false;
    }
}

#[derive(Default, Debug)]
struct InstructionSet {
    instructions: Vec<Instructions>,
    state: State,
}

impl InstructionSet {

    // resets the instruction's "visited" field for re-execution
    fn reset_state(&mut self) {
        for s in self.instructions.iter_mut() {
            s.reset();
        }
        self.state = State::default()
    }

    fn execute(&mut self) -> isize {
        // println!("{:?}", self.state);
        if self.state.cursor >= self.instructions.len() as isize {
            self.state.complete = true;
            return self.state.accumulator;
        }
        let instr = self.instructions[self.state.cursor as usize].borrow_mut();
        // println!("next instructions {:?}", instr);
        if instr.visited {
            return self.state.accumulator;
        }
        let state = &mut self.state;
        instr.op.execute(state);
        instr.visited = true;
        self.execute()
    }

    // this is a brute force crack of fixing the corrupted bit
    fn flip_execute(&mut self) -> isize {
        self.reset_state();
        let mut flippable_index = Vec::new();
        for (i, instr) in self.instructions.iter().enumerate() {
            match instr.op {
                Ops::NOP(_) => {
                    flippable_index.push(i)
                }
                Ops::ACC(_) => {}
                Ops::JMP(_) => {
                    flippable_index.push(i)
                }
            }
        }
        for f in flippable_index {
            // flip the instruction
            self.instructions[f].flip();
            let exec = self.execute();
            // if the task finished, return
            if self.state.complete {
                break
            }
            // reset the state and go to the next flippable index
            self.instructions[f].flip();
            self.reset_state();
        }
        return self.state.accumulator
    }
}

fn main() {
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
    let bar = instructions.flip_execute();
    println!("pt1. {:?}", foo);
    println!("pt1. {:?}", bar)
}
