use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
enum Computation {
    Binop(Op, usize, usize),
    Num(u64),
}

#[derive(Debug, Clone, Copy)]
enum ParsedExpr<'a> {
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
    Num(u64),
}

impl<'a> ParsedExpr<'a> {
    fn binop(op: &'a str, a: &'a str, b: &'a str) -> Self {
        match op {
            "+" => ParsedExpr::Add(a, b),
            "-" => ParsedExpr::Sub(a, b),
            "*" => ParsedExpr::Mul(a, b),
            "/" => ParsedExpr::Div(a, b),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ParsedMonkey<'a> {
    name: &'a str,
    op: ParsedExpr<'a>,
}

impl<'a> ParsedMonkey<'a> {
    fn parse(line: &'a str) -> Self {
        match line.split([' ', ':']).collect_vec().as_slice() {
            [name, _, a, op, b] => ParsedMonkey {
                name,
                op: ParsedExpr::binop(op, a, b),
            },
            [name, _, num] => ParsedMonkey {
                name,
                op: ParsedExpr::Num(num.parse().unwrap()),
            },
            _ => panic!(),
        }
    }
}

pub fn run(input: &str) -> (u64, u64) {
    let monkeys = input.lines().map(ParsedMonkey::parse).collect_vec();
    let indices: FxHashMap<&str, usize> = monkeys
        .iter()
        .enumerate()
        .map(|(a, b)| (b.name, a))
        .collect();
    let ops = monkeys
        .iter()
        .map(|monkey| match monkey.op {
            ParsedExpr::Add(a, b) => Computation::Binop(Op::Add, indices[a], indices[b]),
            ParsedExpr::Sub(a, b) => Computation::Binop(Op::Sub, indices[a], indices[b]),
            ParsedExpr::Mul(a, b) => Computation::Binop(Op::Mul, indices[a], indices[b]),
            ParsedExpr::Div(a, b) => Computation::Binop(Op::Div, indices[a], indices[b]),
            ParsedExpr::Num(n) => Computation::Num(n),
        })
        .collect_vec();

    // Part 1
    let mut results = ops.iter().map(|_| None).collect_vec();
    let mut stack = vec![indices["root"]];
    while let Some(idx) = stack.pop() {
        if results[idx].is_some() {
            continue;
        }
        let op = ops[idx];
        match op {
            Computation::Binop(op, a, b) => {
                if let (Some(aa), Some(bb)) = (results[a], results[b]) {
                    results[idx] = match op {
                        Op::Add => Some(aa + bb),
                        Op::Sub => Some(aa - bb),
                        Op::Mul => Some(aa * bb),
                        Op::Div => Some(aa / bb),
                    };
                } else {
                    stack.push(idx);
                    stack.push(a);
                    stack.push(b);
                }
            }
            Computation::Num(n) => results[idx] = Some(n),
        }
    }
    let res1 = results[indices["root"]].unwrap();

    // Part 2
    let (a, b) = match ops[indices["root"]] {
        Computation::Binop(_, a, b) => (a, b),
        _ => panic!(),
    };
    let human = ops[indices["humn"]];

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
