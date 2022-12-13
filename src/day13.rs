use ascii::AsciiStr;
use itertools::Itertools;
use std::cmp::Ordering;

pub fn run(input: &AsciiStr) -> (usize, usize) {
    let mut lists: Vec<List> = input
        .as_str()
        .lines()
        .filter(|l| !l.is_empty())
        .map(List::parse)
        .collect();

    let mut res1 = 0;
    for (idx, (a, b)) in lists.iter().tuples().enumerate() {
        if a < b {
            res1 += 1 + idx;
        }
    }

    let div_a = List::List(vec![List::List(vec![List::Number(2)])]);
    let div_b = List::List(vec![List::List(vec![List::Number(6)])]);
    lists.push(div_a.clone());
    lists.push(div_b.clone());
    lists.sort_unstable();
    let a_pos = lists.iter().position(|l| *l == div_a).unwrap();
    let b_pos = lists.iter().position(|l| *l == div_b).unwrap();
    let res2 = (1 + a_pos) * (1 + b_pos);

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

// We establish an ordering for lists.
// Using this, we can simply sort the lists in part 2.
impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List::Number(s), List::Number(o)) => s.cmp(o),
            (List::List(s), List::List(o)) => {
                let elem_cmp = s
                    .iter()
                    .zip(o.iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|o| *o != Ordering::Equal);
                let length_cmp = s.len().cmp(&o.len());
                elem_cmp.unwrap_or(length_cmp)
            }
            (List::List(s), o) => s
                .get(0)
                .map_or(Ordering::Less, |s0| s0.cmp(o))
                .then(s.len().cmp(&1)),
            (s, List::List(o)) => o
                .get(0)
                .map_or(Ordering::Greater, |o0| s.cmp(o0))
                .then(1.cmp(&o.len())),
        }
    }
}

// Implementation of PartialOrd is required to implement Ord.
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
        let list = separated_list0(char(','), parse).map(List::List);
        let list = delimited(char('['), list, char(']'));
        let number = decimal.map(List::Number);
        list.or(number).parse(input)
    }

    fn decimal(input: &str) -> IResult<&str, u32> {
        recognize(many1(terminated(
            one_of("0123456789"),
            many0(char::<&str, _>('_')),
        )))
        .map(|o| o.parse::<u32>().unwrap())
        .parse(input)
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
