use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::util::IntoReader;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Or,
}

impl Op {
    fn apply(self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Or => format!("{a}{b}").parse().unwrap(),
        }
    }
}

fn reduce(values: &[usize], ops: &[Op]) -> usize {
    let mut i = 1;
    let mut op_i = 0;
    let mut value = values[0];
    
    while i < values.len() {
        value = ops[op_i].apply(value, values[i]);
        i += 1;
        op_i += 1;
    }

    value
}

// returns true if overflow
fn inc(ops: &mut [Op], part1: bool) -> bool {
    if ops.len() == 0 { return true; }

    let next = if part1 {
        match ops[0] {
            Op::Add => Op::Mul,
            Op::Mul => {
                if inc(&mut ops[1..], part1) {
                    return true;
                }
                Op::Add
            }
            _ => panic!("Or in part1"),
        }
    } else {
        match ops[0] {
            Op::Add => Op::Mul,
            Op::Mul => Op::Or,
            Op::Or => {
                if inc(&mut ops[1..], part1) {
                    return true;
                }
                Op::Add
            }
        }
    };

    ops[0] = next;

    false
}

fn run(input: String, part1: bool) -> Result<()> {
    let mut r = input.reader();

    let tests = r.lines(|r| {
        let result = r.unsigned()?;
        r.text(":")?;
        let values = r.while_ok(|r| {
            r.unsigned()
        });
        Ok((result, values))
    })?;

    let sum: usize = tests
        .into_par_iter()
        .filter(|(result, values)| {
            let mut correct = false;

            let mut ops = vec![Op::Add; values.len() - 1];
    
            loop {
                if reduce(values, &ops) == *result {
                    correct = true;
                    break;
                }
    
                if inc(&mut ops, part1) {
                    break;
                }
            }

            correct
        })
        .map(|(result, _)| result)
        .sum();

    println!("{sum}");

    Ok(())
}

pub fn part1(input: String) -> Result<()> {
    run(input, true)
}

pub fn part2(input: String) -> Result<()> {
    run(input, false)
}
