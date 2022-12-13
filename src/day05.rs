use itertools::Itertools;

struct Instr {
    num: usize,
    src: usize,
    dst: usize,
}

pub fn run(input: &str) -> (String, String) {
    // Split the input into its two parts.
    let (inp_stacks, inp_instrs) = input.split_once("\n\n").unwrap();

    // Initialize the stacks from the bottom line of inputs.
    let mut stack_lines = inp_stacks.lines().rev();
    let mut stacks: Vec<Vec<char>> = stack_lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|_| Vec::new())
        .collect();

    // Go through the remaining lines in top-to bottom while pushing letters to the stacks.
    for line in stack_lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.chars().nth(1 + 4 * i).unwrap_or(' ');
            if c.is_alphabetic() {
                stack.push(c);
            }
        }
    }

    // Create a copy for part 2.
    let mut stacks2 = stacks.clone();

    // Parse instructions.
    let instrs = inp_instrs
        .lines()
        .map(|line| {
            // Split on whitespace and discard irrelevant words.
            let (_, num, _, src, _, dst) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let num = num.parse::<usize>().unwrap();
            let src = src.parse::<usize>().unwrap() - 1;
            let dst = dst.parse::<usize>().unwrap() - 1;
            Instr { num, src, dst }
        })
        .collect_vec();

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
        // Push elements to a temporary vec and pop them one-by-one to preserve order.
        // Not as efficient as extending dst with a whole section of src at a time,
        // but I don't wanna deal with lifetimes.
        for _ in 0..*num {
            tmp.push(stacks2[*src].pop().expect("popped from an empty stack!"));
        }
        while let Some(c) = tmp.pop() {
            stacks2[*dst].push(c);
        }
    }

    let res2: String = stacks2
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect();

    (res1, res2)
}
