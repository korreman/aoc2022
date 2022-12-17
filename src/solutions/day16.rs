use itertools::Itertools;

#[derive(Debug)]
struct PValve<'a> {
    label: &'a str,
    flow: u64,
    tunnels: Vec<&'a str>,
}

impl<'a> PValve<'a> {
    fn parse(line: &'a str) -> Option<Self> {
        let words = line.split([' ', '=', ',', ';']).filter(|word| *word != "");
        let mut words = words.skip(1);
        let label = words.next()?;
        let mut words = words.skip(3);
        let flow = words.next()?.parse().ok()?;
        let tunnels = words.skip(4).collect_vec();
        Some(Self {
            label,
            flow,
            tunnels,
        })
    }
}

#[derive(Debug)]
struct Valve {
    flow: u64,
    tunnels: Vec<usize>,
}

impl Valve {
    fn from_pvalves(pvalves: &[PValve]) -> Vec<Valve> {
        pvalves
            .iter()
            .map(|pvalve| {
                let tunnels = pvalve
                    .tunnels
                    .iter()
                    .map(|tunnel| {
                        pvalves
                            .iter()
                            .position(|dest| dest.label == *tunnel)
                            .unwrap()
                    })
                    .collect_vec();
                Valve {
                    flow: pvalve.flow,
                    tunnels,
                }
            })
            .collect_vec()
    }
}

pub fn run<'a>(input: &'a str) -> (u64, usize) {
    // Parse
    let valves = input
        .lines()
        .map(|l| PValve::parse(l).unwrap())
        .collect_vec();
    let valves = Valve::from_pvalves(valves.as_slice());

    let mut res1 = 0;
    (res1, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "\
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
            Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
            Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
            Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
            Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
            Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
            Valve HH has flow rate=22; tunnel leads to valve GG\n\
            Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
            Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(super::run(input).0, 1651);
    }
}
