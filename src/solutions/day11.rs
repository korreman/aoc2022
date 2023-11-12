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
    test: u64,
    pos: usize,
    neg: usize,
}

pub fn run(input: &str) -> (usize, usize) {
    let make_monkey = |p: &str| {
        let mut lines = p.lines();
        lines.next();
        let items = lines.next()?[18..]
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        let (a, op, b) = lines.next()?[19..].split(' ').collect_tuple().unwrap();
        let test = lines.next()?[21..].parse::<u64>().ok()?;
        let pos = lines.next()?[29..].parse::<usize>().ok()?;
        let neg = lines.next()?[30..].parse::<usize>().ok()?;

        let a = Operand::parse(a);
        let b = Operand::parse(b);
        let op = match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        };
        let op = Binop { a, b, op };
        Some(Monkey {
            inspection_count: 0,
            items,
            op,
            test,
            pos,
            neg,
        })
    };
    let monkeys = input
        .split("\n\n")
        .map(|p| make_monkey(p).unwrap())
        .collect_vec();

    let res1 = play(20, monkeys.clone(), |x| x / 3);

    let modulo: u64 = monkeys.iter().map(|monkey| monkey.test).product();
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
                let target = if item % monkey.test == 0 {
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
        let (res1, _) = run(input);
        assert_eq!(res1, 10605);
    }
}
