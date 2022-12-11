use ascii::AsciiStr;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug)]
enum Operand {
    Old,
    Num(u64),
}

impl Operand {
    fn parse(input: &str) -> Self {
        input
            .parse::<u64>()
            .map_or(Operand::Old, |n| Operand::Num(n))
    }

    fn get(&self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Num(n) => *n,
        }
    }
}

#[derive(Clone, Debug)]
struct Binop {
    a: Operand,
    b: Operand,
    op: Op,
}

impl Binop {
    fn apply(&self, item: u64) -> u64 {
        let a = self.a.get(item);
        let b = self.b.get(item);
        match self.op {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    inspection_count: usize,
    items: Vec<u64>,
    op: Binop,
    test_div: u64,
    pos: usize,
    neg: usize,
}

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let input = input.as_str();
    // Not my proudest work...
    let make_monkey = |p: &str| {
        let mut lines = p.lines();
        drop(lines.next());
        let items = lines
            .next()?
            .strip_prefix("  Starting items: ")?
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec();
        let op = lines
            .next()?
            .strip_prefix("  Operation: new = ")?
            .split(' ')
            .collect_vec();
        let op = match op.as_slice() {
            [a, op, b] => {
                let a = Operand::parse(a);
                let b = Operand::parse(b);
                let op = match *op {
                    "+" => Op::Add,
                    "*" => Op::Mul,
                    _ => panic!(),
                };
                Binop { a, b, op }
            }
            _ => panic!(),
        };
        let test_div = lines
            .next()?
            .strip_prefix("  Test: divisible by ")?
            .parse::<u64>()
            .ok()?;
        let pos = lines
            .next()?
            .strip_prefix("    If true: throw to monkey ")?
            .parse::<usize>()
            .ok()?;
        let neg = lines
            .next()?
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .ok()?;
        Some(Monkey {
            inspection_count: 0,
            items,
            op,
            test_div,
            pos,
            neg,
        })
    };
    let monkeys = input
        .split("\n\n")
        .map(|p| make_monkey(p).unwrap())
        .collect_vec();

    let res1 = play(20, monkeys.clone(), |x| x / 3);

    let modulo: u64 = monkeys.iter().map(|monkey| monkey.test_div).product();
    let res2 = play(10000, monkeys, |x| x % modulo);

    (res1, res2)
}

fn play<F>(steps: usize, mut monkeys: Vec<Monkey>, worry_reduction: F) -> usize
where
    F: Fn(u64) -> u64,
{
    for _ in 0..steps {
        let monkeys_target: *mut Vec<Monkey> = &mut monkeys;
        for monkey in monkeys.iter_mut() {
            for item in monkey.items.iter() {
                monkey.inspection_count += 1;
                let item = worry_reduction(monkey.op.apply(*item));
                let target = if item % monkey.test_div == 0 {
                    monkey.pos
                } else {
                    monkey.neg
                };
                unsafe {
                    (*monkeys_target)[target].items.push(item);
                }
            }
            monkey.items.clear();
        }
    }
    let mut inspection_counts = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect_vec();
    inspection_counts.sort_unstable();
    inspection_counts.pop().unwrap() * inspection_counts.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::run;
    use ascii::AsciiStr;

    #[test]
    fn test() {
        let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let (res1, _) = run(AsciiStr::from_ascii(input).unwrap());
        assert_eq!(res1, 10605);
    }
}
