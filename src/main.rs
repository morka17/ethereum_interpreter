#!(allow_deadcode)

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum VMError {
    UNDERFLOW,
    OVERFLOW,
    BADOP,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Word(u32);

#[derive(Default)]
struct Stack {
    pub sp: usize,
    pub values: Vec<Word>,
}

impl Stack {
    // fn new() ->Self {
    //     Stack {
    //         sp: 0,
    //         values: vec![Word::default(); 1024]
    //     }
    // }
    fn add(&mut self) -> Result<(), VMError> {
        let a = self.values.pop().ok_or(VMError::UNDERFLOW)?;
        let b = self.values.pop().ok_or(VMError::UNDERFLOW)?;
        self.values.push(Word(a.0 + b.0));
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instructions {
    Add,
    Push1(u8),
}

type Instruction = u8;

const OP_ADD: Instruction = 0x01;
const OP_PUSH1: Instruction = 0x02;

fn execute(
    stack: &mut Stack,
    op: Instruction,
    more: &mut std::slice::Iter<Instruction>,
) -> Result<(), VMError> {

    println!("{:?}", op);
    match op {
        OP_ADD => {
            let a = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            let b = stack.values.pop().ok_or(VMError::UNDERFLOW)?;
            stack.values.push(Word(a.0 + b.0));
        }
        OP_PUSH1 => stack
            .values
            .push(Word(*more.next().ok_or(VMError::BADOP)? as u32)),
        _ =>  {
            return Err(VMError::BADOP)
        }
    }
    Ok(())
}

fn play_ground(stack: &mut Stack) -> Result<(), VMError> {
    let mut instructions: Vec<Instruction> = vec![OP_PUSH1, 1, OP_PUSH1, 2, OP_ADD];

    let mut iter = instructions.iter();

    while let Some(op) = iter.next() {
        let error = execute(stack, *op, &mut iter)?;
    }

    Ok(())
}

fn main() {
    let mut stack = Stack::default();
    match play_ground(&mut stack) {
        Ok(()) => println!("DONE! {:?}", stack.values),
        Err(err) => println!("ERROR: {:?}", err),
    }
}

// MVP
// Take an instruction input Buffer
// pop in a loop
// Execute

// Add 0x1
// push Push 0x60
