use itertools::Itertools;
// The optimization:
// Grow each diamond by 1, and find its intersections with other diamonds.
// Check all of these intersections against the original diamonds.

struct Diamond {
    x: i32,
    y: i32,
    size: u32,
}

fn dist(a: (i32, i32), b: (i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl Diamond {
    fn infer(pos: (i32, i32), beacon: (i32, i32)) -> Self {
        let size = dist(pos, beacon);
        Self {
            x: pos.0,
            y: pos.1,
            size,
        }
    }

    fn to_slice(&self, row: i32) -> Option<Slice> {
        let y_dist = self.y.abs_diff(row);
        if y_dist > self.size {
            None
        } else {
            let slice_size = self.size - y_dist;
            Some(Slice::new(
                self.x - slice_size as i32,
                self.x + slice_size as i32,
            ))
        }
    }

    fn contains(&self, pos: (i32, i32)) -> bool {
        let dist = pos.0.abs_diff(self.x) + pos.1.abs_diff(self.y);
        dist <= self.size
    }

    // Produces 8 points that are possibly intersections of the diamonds (grown by 1).
    // We effectively treat each line of the diamond as a function,
    // and solve the intersections of these.
    fn pseudo_intersections(&self, other: &Self) -> [(i32, i32); 8] {
        let p = |a, b, c, d| {
            let s = self.x + a * self.y + b * (self.size as i32 + 1);
            let o = other.x + c * other.y + d * (other.size as i32 + 1);
            let x = (s + o) / 2;
            (x, s - x)
        };
        #[rustfmt::skip]
        let res = [
            p( 1,  1, -1,  1),
            p( 1,  1, -1, -1),
            p( 1, -1, -1,  1),
            p( 1, -1, -1, -1),

            p(-1,  1,  1,  1),
            p(-1, -1,  1, -1),
            p(-1,  1,  1,  1),
            p(-1, -1,  1, -1),
        ];
        res
    }
}

struct Boundary<'a> {
    diamond: &'a Diamond,
    progress: u32,
}

impl<'a> Iterator for Boundary<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.diamond.size + 1;
        let res = {
            let qprog = (self.progress % size) as i32;
            (size as i32 - qprog, qprog)
        };
        let res = match self.progress / self.diamond.size {
            0 => Some((self.diamond.x - res.0, self.diamond.y + res.1)),
            1 => Some((self.diamond.x + res.1, self.diamond.y - res.0)),
            2 => Some((self.diamond.x + res.0, self.diamond.y - res.1)),
            3 => Some((self.diamond.x - res.1, self.diamond.y - res.0)),
            _ => None,
        };
        self.progress += 1;
        res
    }
}

struct Slice {
    start: i32,
    end: i32,
}

impl Slice {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    fn len(&self) -> i32 {
        self.end - self.start + 1
    }

    fn overlap(&self, other: &Slice) -> i32 {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if end >= start {
            Slice { start, end }.len()
        } else {
            0
        }
    }

    fn contains(&self, point: i32) -> bool {
        (self.start..=self.end).contains(&point)
    }
}

pub fn run(input: &str) -> (i32, u64) {
    #[cfg(test)]
    let line = 10;
    #[cfg(not(test))]
    let line = 2_000_000;
    let search_space = line * 2;

    // Parse
    let sensors: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .map(|l| {
            let (_, _, sx, sy, _, _, _, _, bx, by) = l.split(' ').collect_tuple().unwrap();
            let sx = sx[2..].strip_suffix(',').unwrap().parse().unwrap();
            let sy = sy[2..].strip_suffix(':').unwrap().parse().unwrap();
            let bx = bx[2..].strip_suffix(',').unwrap().parse().unwrap();
            let by = by[2..].parse().unwrap();
            ((sx, sy), (bx, by))
        })
        .collect();

    let diamonds = sensors
        .iter()
        .map(|(sensor, beacon)| Diamond::infer(*sensor, *beacon))
        .collect_vec();

    // Collect slices
    let mut slices: Vec<Slice> = diamonds
        .iter()
        .filter_map(|d| d.to_slice(line))
        .collect_vec();

    // Sum non-overlapping parts of slices using a rolling boundary.
    let mut res1 = 0;
    slices.sort_unstable_by_key(|s| s.start);
    let mut already_counted = Slice::new(i32::MIN, i32::MIN);
    for slice in &slices {
        res1 += slice.len();
        res1 -= slice.overlap(&already_counted);
        already_counted.end = already_counted.end.max(slice.end);
    }

    // Subtract the beacons that overlap with a slice.
    let mut beacons = sensors.iter().map(|(_, b)| *b).collect_vec();
    beacons.sort_unstable();
    beacons.dedup();
    for (bx, by) in beacons {
        if by == line && slices.iter().any(|slice| slice.contains(bx)) {
            res1 -= 1;
        }
    }

    let mut beacon = None;
    for point in diamonds.iter().tuple_combinations().flat_map(|(d1, d2)| {
        d1.pseudo_intersections(d2)
            .into_iter()
            .filter(|p| dist(*p, (d1.x, d1.y)) == d1.size + 1)
    }) {
        let inside_space =
            point.0 >= 0 && point.0 <= search_space && point.1 >= 0 && point.1 <= search_space;
        if inside_space && diamonds.iter().all(|d| !d.contains(point)) {
            beacon = Some(point);
            break;
        }
    }

    let beacon = beacon.expect("no beacon found");
    let res2 = beacon.0 as u64 * 4_000_000 + beacon.1 as u64;
    (res1, res2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = "\
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
            Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
            Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
            Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
            Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
            Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
            Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
            Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
            Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
            Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
            Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
            Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
            Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
            Sensor at x=20, y=1: closest beacon is at x=15, y=3\n\
        ";
        assert_eq!(super::run(input), (26, 56000011));
    }
}
