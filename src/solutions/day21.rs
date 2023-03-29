use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
enum PExpr<'a> {
    Binop(Op, &'a str, &'a str),
    Num(i64),
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
    Num(i64),
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

pub fn run(input: &str) -> (i64, i64) {
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
    let mut results: Vec<Option<i64>> = monkeys.iter().map(|_| None).collect_vec();
    let mut has_humn = monkeys.iter().map(|_| false).collect_vec();
    has_humn[indices["humn"]] = true;
    let mut stack = vec![indices["root"]];
    let mut humn_path: Vec<(Op, bool, f64)> = Vec::new();
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
                    if has_humn[a] {
                        humn_path.push((op, false, bb as f64));
                    }
                    if has_humn[b] {
                        humn_path.push((op, true, aa as f64));
                    }
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
    // The graph of monkeys is a tree.
    // We built a path of operations reaching humn in the previous step.
    // Now we simply retrieve the other side of the equation
    // and apply the inverse operation for each step in the path.
    let (_, _, equalee) = humn_path.pop().unwrap();
    let res2 = humn_path
        .iter()
        .rev()
        .fold(equalee, |acc, (op, flip, num)| match op {
            Op::Add => acc - num,
            Op::Sub => {
                if *flip {
                    num - acc
                } else {
                    acc + num
                }
            }
            Op::Mul => acc / num,
            Op::Div => {
                if *flip {
                    num / acc
                } else {
                    acc * num
                }
            }
        });

    (res1, res2 as i64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32";
        assert_eq!(super::run(input), (152, 301));
    }
}
