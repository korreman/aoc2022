fn read_snafu(number: &str) -> i64 {
    let mut result = 0;
    for c in number.chars() {
        result *= 5;
        match c {
            '2' => result += 2,
            '1' => result += 1,
            '0' => (),
            '-' => result -= 1,
            '=' => result -= 2,
            _ => panic!(),
        }
    }
    result
}

fn print_snafu(mut number: i64) -> String {
    let mut result = String::new();
    while number != 0 {
        let rem = number % 5;
        number /= 5;
        match rem {
            0 => result.push('0'),
            1 => result.push('1'),
            2 => result.push('2'),
            3 => {number += 1; result.push('=')},
            4 => {number += 1; result.push('-')}
            _ => unreachable!(),
        }
    }
    result.chars().rev().collect()
}

pub fn run(input: &str) -> (String, &'static str) {
    let sum: i64 = input.lines().map(read_snafu).sum();
    let res1 = print_snafu(sum);
    (res1, "")
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_blizzards() {
        let input = "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122";
        assert_eq!(run(input).0.as_str(), "2=-1=0");
    }
}
