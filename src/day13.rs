use ascii::AsciiStr;
use std::cmp::Ordering;

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let mut res1 = 0;
    for (idx, line) in input.as_str().split("\n\n").enumerate() {
        let (a, b) = line.split_once('\n').unwrap();
        let (a, b) = (List::parse(a), List::parse(b));
        if a < b {
            res1 += 1 + idx;
        }
    }
    let mut lists: Vec<List> = input
        .as_str()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| List::parse(l))
        .collect();
    let div_a = List::List(vec![List::List(vec![List::Number(2)])]);
    let div_b = List::List(vec![List::List(vec![List::Number(6)])]);
    lists.push(div_a.clone());
    lists.push(div_b.clone());
    lists.sort();
    let res2 = (1 + lists.iter().position(|l| *l == div_a).unwrap())
        * (1 + lists.iter().position(|l| *l == div_b).unwrap());
    (res1, res2)
}

#[derive(PartialEq, Eq, Clone)]
pub enum List {
    List(Vec<List>),
    Number(u32),
}

impl List {
    fn parse(list: &str) -> Self {
        parser::parse(list).unwrap().1
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Number(s), List::Number(o)) => s.cmp(o),
            (List::List(s), List::List(o)) => s
                .iter()
                .zip(o.iter())
                .filter_map(|(a, b)| {
                    let o = a.cmp(b);
                    if o != Ordering::Equal {
                        Some(o)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap_or(s.len().cmp(&o.len())),
            (s, List::Number(o)) => s.cmp(&List::List(vec![List::Number(*o)])),
            (List::Number(s), o) => List::List(vec![List::Number(*s)]).cmp(o),
        }
    }
}

mod parser {
    use super::List;
    use nom::{
        character::complete::{char, one_of},
        combinator::recognize,
        multi::{many0, many1, separated_list0},
        sequence::{delimited, terminated},
        IResult, Parser,
    };

    pub fn parse(input: &str) -> IResult<&str, List> {
        let list = separated_list0(char(','), parse).map(|l| List::List(l));
        let list = delimited(char('['), list, char(']'));
        let number = decimal.map(|n| List::Number(n));
        list.or(number).parse(input)
    }

    fn decimal(input: &str) -> IResult<&str, u32> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
            .map(|(i, o)| (i, o.parse::<u32>().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use ascii::AsciiStr;

    #[test]
    fn test() {
        let input = "\
            [1,1,3,1,1]\n\
            [1,1,5,1,1]\n\
            \n\
            [[1],[2,3,4]]\n\
            [[1],4]\n\
            \n\
            [9]\n\
            [[8,7,6]]\n\
            \n\
            [[4,4],4,4]\n\
            [[4,4],4,4,4]\n\
            \n\
            [7,7,7,7]\n\
            [7,7,7]\n\
            \n\
            []\n\
            [3]\n\
            \n\
            [[[]]]\n\
            [[]]\n\
            \n\
            [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
            [1,[2,[3,[4,[5,6,0]]]],8,9]";
        let res = super::run(AsciiStr::from_ascii(input).unwrap());
        assert_eq!(res, (13, 140));
    }
}
