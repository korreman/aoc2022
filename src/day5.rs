use itertools::Itertools;

struct Instr {
    num: usize,
    src: usize,
    dst: usize,
}

pub fn run(input: &str) -> (String, String) {
    let (inp_stacks, inp_instrs) = input.split("\n\n").collect_tuple().unwrap();

    let mut stack_lines = inp_stacks.lines().rev();
    let mut stacks: Vec<Vec<char>> = stack_lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|_| Vec::new())
        .collect();

    for line in stack_lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.chars().nth(1 + 4 * i).unwrap_or(' ');
            if c.is_alphabetic() {
                stack.push(c);
            }
        }
    }
    let mut stacks2 = stacks.clone();

    let instrs = inp_instrs.lines().map(|line| {
        let (_, num, _, src, _, dst) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let num = num.parse::<usize>().unwrap();
        let src = src.parse::<usize>().unwrap() - 1;
        let dst = dst.parse::<usize>().unwrap() - 1;
        Instr { num, src, dst }
    }).collect_vec();


    // Part 1
    for Instr { num, src, dst } in &instrs {
        for _ in 0..*num {
            let c = stacks[*src].pop().expect("popped from an empty stack!");
            stacks[*dst].push(c);
        }
    }

    let res1: String = stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect();

    // Part 2
    let mut tmp = Vec::new();
    for Instr { num, src, dst } in &instrs {
        for _ in 0..*num {
            tmp.push(stacks2[*src].pop().expect("popped from an empty stack!"));
        }
        for c in tmp.iter().rev() {
            stacks2[*dst].push(*c);
        }
        tmp.clear();
    }

    let res2: String = stacks2
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect();

    (res1, res2)
}
