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
    op: Binop,
    test: u64,
    pos: usize,
    neg: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Item {
    monkey: usize,
    worry: u64,
}

pub fn run(input: &str) -> (usize, usize) {
    let make_monkey = |p: &str| {
        let mut lines = p.lines();
        let monkey: usize = lines.next()?[7..].strip_suffix(':')?.parse().ok()?;
        let items: Vec<Item> = lines.next()?[18..]
            .split(", ")
            .map(|n| Item {
                monkey,
                worry: n.parse().unwrap(),
            })
            .collect();
        let (a, op, b) = lines.next()?[19..].split(' ').collect_tuple()?;
        let test: u64 = lines.next()?[21..].parse().ok()?;
        let pos: usize = lines.next()?[29..].parse().ok()?;
        let neg: usize = lines.next()?[30..].parse().ok()?;

        let a = Operand::parse(a);
        let b = Operand::parse(b);
        let op = match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        };
        let op = Binop { a, b, op };
        Some((Monkey { op, test, pos, neg }, items))
    };

    let (mut monkeys, itemss): (Vec<Monkey>, Vec<Vec<Item>>) =
        input.split("\n\n").map(|p| make_monkey(p).unwrap()).unzip();
    let mut items = itemss
        .into_iter()
        .flat_map(|items| items.into_iter())
        .collect_vec();
    let modulo: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    let mut items2 = items.clone();
    let res1 = play(20, monkeys.as_mut_slice(), &mut items, |x| x / 3);
    let res2 = play(10000, monkeys.as_mut_slice(), &mut items2, |x| x % modulo);

    (res1, res2)
}

fn play<F>(steps: usize, monkeys: &[Monkey], items: &mut [Item], worry_reduction: F) -> usize
where
    F: Fn(u64) -> u64,
{
    let mut inspections: Vec<usize> = monkeys.iter().map(|_| 0).collect_vec();
    for item in items {
        for _ in 0..steps {
            loop {
                let m = &monkeys[item.monkey];
                inspections[item.monkey] += 1;
                item.worry = worry_reduction(m.op.apply(item.worry));
                let new_monkey = if item.worry % m.test == 0 {
                    m.pos
                } else {
                    m.neg
                };
                let stop = new_monkey <= item.monkey;
                item.monkey = new_monkey;
                if stop {
                    break;
                }
            }
        }
    }
    inspections.sort_unstable();
    inspections.pop().unwrap() * inspections.pop().unwrap()
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
