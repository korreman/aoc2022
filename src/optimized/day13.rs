pub fn run(input: &str) -> (usize, u32) {
    let mut res1 = 0;
    let mut pos_a = 0;
    let mut pos_b = 1;
    for (idx, pair) in input.split_terminator("\n\n").enumerate() {
        let (left, right) = pair.split_once('\n').unwrap();
        let (left, right) = (left.as_bytes(), right.as_bytes());
        if ordered(left, right) {
            res1 += 1 + idx;
        }
        if ordered(left, b"[[2]]") {
            pos_a += 1;
        }
        if ordered(right, b"[[2]]") {
            pos_a += 1;
        }
        if ordered(left, b"[[6]]") {
            pos_b += 1;
        }
        if ordered(right, b"[[6]]") {
            pos_b += 1;
        }
    }
    let res2 = (pos_a + 1) * (pos_b + 1);
    (res1, res2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Open,
    Num(u8),
    Close,
    Eof,
}

struct Stream<'a> {
    head: Token,
    pending_closes: usize,
    tail: &'a [u8],
}

impl<'a> Stream<'a> {
    fn new(stream: &'a [u8]) -> Self {
        let mut res = Self { head: Token::Eof, pending_closes: 0, tail: stream };
        res.step();
        res
    }

    fn nest(&mut self) {
        self.pending_closes += 1;
    }

    fn step(&mut self) {
        if self.pending_closes > 0 {
            self.pending_closes -= 1;
            self.head = Token::Close;
            return;
        }
        if self.tail.is_empty() {
            self.head = Token::Eof;
            return;
        }
        while self.tail[0] == b',' {
            self.tail = &self.tail[1..];
        }
        if self.tail.get(0..2) == Some(&[b'1', b'0']) {
            self.tail = &self.tail[2..];
            self.head = Token::Num(b'9' + 1);
            return;
        }
        self.head = match self.tail[0] {
            b'[' => Token::Open,
            b']' => Token::Close,
            c if c.is_ascii_digit() => Token::Num(c),
            _ => panic!("unrecognized character: {}", self.tail[0]),
        };
        self.tail = &self.tail[1..];
    }
}

fn ordered(lhs: &[u8], rhs: &[u8]) -> bool {
    let mut l = Stream::new(lhs);
    let mut r = Stream::new(rhs);
    loop {
        match (l.head, r.head) {
            (Token::Eof, Token::Eof) => return true,
            (x, y) if x == y => {
                l.step();
                r.step();
            }
            (Token::Num(x), Token::Num(y)) => return x <= y,
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
            _ => panic!("unexpected token pair"),
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
