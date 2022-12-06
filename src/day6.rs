use std::collections::VecDeque;

pub fn run(input: &str) -> (usize, usize) {
    let res1 = task(input, 4);
    let res2 = task(input, 14);
    (res1, res2)
}

fn task(input: &str, window_size: usize) -> usize {
    let mut window: VecDeque<char> = input.chars().take(window_size - 1).collect();
    let mut counter = window_size - 1;
    for c in input.chars().skip(window_size - 1) {
        counter += 1;
        window.push_back(c);

        if window
            .iter()
            .all(|a| window.iter().filter(|&b| b == a).count() == 1)
        {
            return counter;
        }
        window.pop_front().expect("popped empty window!");
    }
    panic!("no marker detected!")
}
