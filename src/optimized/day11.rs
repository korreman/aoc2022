use itertools::Itertools;

use crate::util::cycles::CycleFinder;

#[derive(Clone, Debug)]
struct Monkey {
    op: Op,
    test: u64,
    positive: usize,
    negative: usize,
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Pow2,
}

impl Op {
    fn apply(self, lhs: u64) -> u64 {
        match self {
            Op::Add(rhs) => lhs + rhs,
            Op::Mul(rhs) => lhs * rhs,
            Op::Pow2 => lhs * lhs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Item {
    monkey: usize,
    worry: u64,
}

pub fn run(input: &str) -> (u64, u64) {
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

        let op = match (a, op, b) {
            ("old", "+", "old") => Op::Mul(2), // NOTE: I don't think this one is necessary.
            ("old", "*", "old") => Op::Pow2,
            ("old", "+", n) => Op::Add(n.parse().ok()?),
            ("old", "*", n) => Op::Mul(n.parse().ok()?),
            _ => return None,
        };
        Some((
            Monkey {
                op,
                test,
                positive: pos,
                negative: neg,
            },
            items,
        ))
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

fn play<F: Fn(u64) -> u64>(
    rounds: u64,
    monkeys: &[Monkey],
    items: &mut [Item],
    worry_reduction: F,
) -> u64 {
    let mut inspections: Vec<u64> = monkeys.iter().map(|_| 0).collect_vec();
    for item in items {
        let mut finder = CycleFinder::new();
        let mut rounds_left = rounds;
        let repeated = 'outer: loop {
            if rounds_left == 0 {
                break None;
            }
            loop {
                let monkey = &monkeys[item.monkey];
                inspections[item.monkey] += 1;
                item.worry = worry_reduction(monkey.op.apply(item.worry));

                if let Some(repeated) = finder.push((item.monkey, item.worry % monkey.test)) {
                    break 'outer Some(repeated);
                }

                let new_monkey = if item.worry % monkey.test == 0 {
                    monkey.positive
                } else {
                    monkey.negative
                };
                let stop = new_monkey <= item.monkey;
                item.monkey = new_monkey;
                if stop {
                    break;
                }
            }
            rounds_left -= 1;
        };
        if let Some(repeated) = repeated {
            // Find out how many rounds we advance per repetition.
            let mut rep_rounds = repeated
                .iter()
                .tuple_windows()
                .filter(|((a, _), (b, _))| b <= a)
                .count() as u64;
            // also count the beginning of a new repetition
            if repeated.first().unwrap() <= repeated.last().unwrap() {
                rep_rounds += 1;
            }

            // 2. perform as many repetitions as possible
            let max_repetitions = (rounds_left - 1) / rep_rounds;
            for &(seq_monkey, _) in repeated {
                inspections[seq_monkey] += max_repetitions;
            }
            rounds_left -= rep_rounds * max_repetitions;

            // 3. handle the tail end
            if repeated.first().unwrap() <= repeated.last().unwrap() && rounds_left > 0 {
                rounds_left -= 1;
            }
            for ((monkey_a, _), (monkey_b, _)) in repeated.iter().tuple_windows() {
                if rounds_left == 0 {
                    break;
                }
                inspections[*monkey_a] += 1;
                if monkey_b <= monkey_a {
                    rounds_left -= 1;
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
