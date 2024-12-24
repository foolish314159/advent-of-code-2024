use std::fs::read_to_string;

use itertools::{repeat_n, Itertools};

// Normal function version, equivalent of python itertools product with repeat
fn product<T>(v: &[T], repeat: usize) -> impl Iterator<Item = Vec<&T>> {
    repeat_n(v, repeat).multi_cartesian_product()
}

// Macro version, equivalent of python itertools product with repeat
macro_rules! product {
    ($it:expr, $rep:expr) => {
        vec![$it; $rep].into_iter().multi_cartesian_product()
    };
}

pub fn calibration_result(filename: &str, operators: &[Operator]) -> u64 {
    parse_equation_inputs(filename)
        .iter()
        .filter_map(|input| Some(get_equation(input, operators)?.calculate()))
        .sum()
}

fn parse_equation_inputs(filename: &str) -> Vec<EquationInput> {
    read_to_string(filename)
        .unwrap_or(String::from(""))
        .lines()
        // Lines have the format "<res>: <op1> <op2> ... <opN>"
        .filter_map(|line| {
            let mut parts = line.split(":");
            let res = parts.next()?.parse::<u64>().ok()?;
            let ops = parts
                .next()?
                .trim()
                .split_whitespace()
                .filter_map(|e| e.parse::<u64>().ok())
                .collect_vec();

            Some(EquationInput {
                result: res,
                operands: ops,
            })
        })
        .collect_vec()
}

// Returns the first possible Equation given EquationInput and set of Operators
// Returns None if not possible
fn get_equation(input: &EquationInput, operators: &[Operator]) -> Option<Equation> {
    for op in product!(operators, input.operands.len() - 1) {
        let eq = Equation {
            operands: input.operands.clone(),
            operators: op.into_iter().cloned().collect(),
        };

        if input.result == eq.calculate() {
            return Some(eq);
        }
    }

    None
}

impl Equation {
    fn calculate(&self) -> u64 {
        self.operators
            .iter()
            .enumerate()
            .fold(self.operands[0], |acc, (i, op)| {
                match op {
                    Operator::Add => acc + self.operands[i + 1],
                    Operator::Mul => acc * self.operands[i + 1],
                    Operator::Concat => {
                        // res = format!("{}{}", res, self.operands[i + 1])
                        //     .parse::<u64>()
                        //     .unwrap_or(res);
                        acc * 10u64.pow(self.operands[i + 1].ilog10() + 1) + self.operands[i + 1]
                    }
                }
            })
    }
}

struct EquationInput {
    result: u64,
    operands: Vec<u64>,
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Mul,
    Concat,
}

struct Equation {
    operands: Vec<u64>,
    operators: Vec<Operator>,
}
