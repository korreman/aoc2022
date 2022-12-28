use std::collections::VecDeque;

use fxhash::FxHashMap;
use itertools::Itertools;

use crate::util::graph::{GraphImpl, VecGraph};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
enum PExpr<'a> {
    Binop(Op, &'a str, &'a str),
    Num(u64),
}

impl<'a> PExpr<'a> {
    fn binop(op: &'a str, a: &'a str, b: &'a str) -> Self {
        match op {
            "+" => PExpr::Binop(Op::Add, a, b),
            "-" => PExpr::Binop(Op::Sub, a, b),
            "*" => PExpr::Binop(Op::Mul, a, b),
            "/" => PExpr::Binop(Op::Div, a, b),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Expr {
    Binop(Op, usize, usize),
    Num(u64),
}

#[derive(Debug, Clone, Copy)]
struct PMonkey<'a> {
    name: &'a str,
    expr: PExpr<'a>,
}

impl<'a> PMonkey<'a> {
    fn parse(line: &'a str) -> Self {
        match line.split([' ', ':']).collect_vec().as_slice() {
            [name, _, a, op, b] => PMonkey {
                name,
                expr: PExpr::binop(op, a, b),
            },
            [name, _, num] => PMonkey {
                name,
                expr: PExpr::Num(num.parse().unwrap()),
            },
            _ => panic!(),
        }
    }
}

pub fn run(input: &str) -> (u64, u64) {
    let monkeys = input.lines().map(PMonkey::parse).collect_vec();
    let indices: FxHashMap<&str, usize> = monkeys
        .iter()
        .enumerate()
        .map(|(a, b)| (b.name, a))
        .collect();
    let monkeys = monkeys
        .iter()
        .map(|monkey| match monkey.expr {
            PExpr::Binop(op, a, b) => Expr::Binop(op, indices[a], indices[b]),
            PExpr::Num(n) => Expr::Num(n),
        })
        .collect_vec();

    // Part 1
    let mut results = monkeys.iter().map(|_| None).collect_vec();
    let mut has_humn = monkeys.iter().map(|_| false).collect_vec();
    has_humn[indices["humn"]] = true;
    let mut stack = vec![indices["root"]];
    while let Some(idx) = stack.pop() {
        if results[idx].is_some() {
            continue;
        }
        let op = monkeys[idx];
        match op {
            Expr::Binop(op, a, b) => {
                if let (Some(aa), Some(bb)) = (results[a], results[b]) {
                    results[idx] = match op {
                        Op::Add => Some(aa + bb),
                        Op::Sub => Some(aa - bb),
                        Op::Mul => Some(aa * bb),
                        Op::Div => Some(aa / bb),
                    };
                    has_humn[idx] = has_humn[a] || has_humn[b];
                } else {
                    stack.push(idx);
                    stack.push(a);
                    stack.push(b);
                }
            }
            Expr::Num(n) => results[idx] = Some(n),
        }
    }
    let res1 = results[indices["root"]].unwrap();

    // Part 2
    let (a, b) = match monkeys[indices["root"]] {
        Expr::Binop(_, a, b) => (a, b),
        _ => panic!(),
    };
    let human = monkeys[indices["humn"]];
    println!("{}, {}", has_humn[a], has_humn[b]);
    // Starting at humn, work your way up the expression DAG and build a series of modifications.
    // When you reach either a or b, perform all those modifications in reverse on the opposite.

    (res1, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32";
        assert_eq!(super::run(input), (152, 301));
    }
}
