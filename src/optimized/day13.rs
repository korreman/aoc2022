use itertools::Itertools;

pub fn run(input: &str) -> (usize, u32) {
    let mut num_ordered = 0;

    let mut pos_a = 1;
    let mut pos_b = 2;
    let a = b"[[2]]";
    let b = b"[[6]]";

    for (idx, (left, right)) in input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .tuples()
        .enumerate()
    {
        if ordered(left, right) {
            num_ordered += 1 + idx;
        }
        if ordered(right, a) {
            pos_a += 1;
            pos_b += 1;
        } else if ordered(right, b) {
            pos_b += 1;
        }
        if ordered(left, a) {
            pos_a += 1;
            pos_b += 1;
        } else if ordered(left, b) {
            pos_b += 1;
        }
    }

    (num_ordered, pos_a * pos_b)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Num(u8),
    Close,
    End,
}

struct Stream<'a> {
    head: Token,
    pending_closes: usize,
    tail: &'a [u8],
}

impl<'a> Stream<'a> {
    fn new(stream: &'a [u8]) -> Self {
        let mut res = Self { head: Token::End, pending_closes: 0, tail: stream };
        res.step();
        res
    }

    #[inline(always)]
    fn nest(&mut self) {
        self.pending_closes += 1;
    }

    #[inline(always)]
    fn step(&mut self) {
        if self.pending_closes > 0 {
            self.pending_closes -= 1;
            self.head = Token::Close;
            return;
        }
        if self.tail.is_empty() {
            self.head = Token::End;
            return;
        }
        while self.tail[0] == b',' {
            self.tail = &self.tail[1..];
        }
        if self.tail.get(0..2) == Some(b"10") {
            self.tail = &self.tail[2..];
            self.head = Token::Num(b'9' + 1);
            return;
        }
        self.head = match self.tail[0] {
            b'[' => Token::Open,
            b']' => Token::Close,
            c => Token::Num(c),
        };
        self.tail = &self.tail[1..];
    }
}

fn ordered(lhs: &[u8], rhs: &[u8]) -> bool {
    let mut l = Stream::new(lhs);
    let mut r = Stream::new(rhs);
    loop {
        match (l.head, r.head) {
            (Token::End, Token::End) => return true,
            (x, y) if x == y => {
                l.step();
                r.step();
            }
            (Token::Num(x), Token::Num(y)) => return x < y,
            (Token::Open, Token::Num(_)) => {
                l.step();
                r.nest();
            }
            (Token::Num(_), Token::Open) => {
                r.step();
                l.nest();
            }
            (Token::Close, _) => return true,
            (_, Token::Close) => return false,
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(ordered(b"[1,1,3,1,1]", b"[1,1,5,1,1]"));
        assert!(ordered(b"[[1],[2,3,4]]", b"[[1],4]"));
        assert!(!ordered(b"[9]", b"[[8,7,6]]"));
        assert!(ordered(b"[[4,4],4,4]", b"[[4,4],4,4,4]"));
        assert!(!ordered(b"[7,7,7,7]", b"[7,7,7]"));
        assert!(ordered(b"[]", b"[3]"));
        assert!(!ordered(b"[[[]]]", b"[[]]"));
        assert!(!ordered(
            b"[1,[2,[3,[4,[5,6,7]]]],8,9]",
            b"[1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));
        assert!(ordered(b"[3,[5]]", b"[[3,[4]]]"));
    }

    #[test]
    fn sample() {
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
        let res = super::run(input);
        assert_eq!(res, (13, 140));
    }
}
