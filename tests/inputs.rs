// We should test each solution against the corresponding data in each profile.
// The question is if we should have a unique test for each solution,
// or if some tests should run multiple solutions.
//
// The profiles are dynamic, so I guess those can be dynamic.
// The tests will have to be per-solution.

mod inputs {
    macro_rules! input {
        ($name:ident, $submod:ident) => {
            #[test]
            fn $name() {
                // TODO: move directory finding to util?
                let dir = std::fs::read_dir("data").expect("no data directory");
                let mut profiles: Vec<_> = dir.map(|x| x.unwrap()).collect();
                profiles.sort_by_key(|profile| profile.path());
                for entry in profiles {
                    let profile_name = entry
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned();
                    let mut input_path = entry.path().to_path_buf();
                    input_path.push("inputs");
                    input_path.push(stringify!($name).to_owned() + ".txt");

                    let mut expected_path = entry.path().to_path_buf();
                    expected_path.push("solutions");
                    expected_path.push(stringify!($name).to_owned() + ".txt");

                    let input =
                        std::fs::read_to_string(input_path).expect("couldn't read input file");
                    let (actual1, actual2) = aoc2022::$submod::$name::run(input.as_str());

                    let expected = std::fs::read_to_string(expected_path).ok();
                    if let Some((expected1, expected2)) =
                        expected.as_ref().and_then(|x| x.split_once("\n\n"))
                    {
                        custom_assert(
                            &(profile_name.clone() + " part 1"),
                            &actual1.to_string(),
                            expected1,
                        );
                        custom_assert(&(profile_name + " part 2"), &actual2.to_string(), expected2);
                    } else if let Some(expected1) = expected {
                        custom_assert(
                            &(profile_name + " part 1"),
                            &actual1.to_string(),
                            &expected1,
                        );
                    }
                }
            }
        };
    }

    fn custom_assert(profile: &str, actual: &str, expected: &str) {
        let actual = actual.trim_end();
        let expected = expected.trim_end();
        assert!(
            actual == expected,
            "wrong result in {profile}\nexpected: {expected}\n  actual: {actual}"
        );
    }

    input!(day01, optimized);
    input!(day02, solutions);
    input!(day03, solutions);
    input!(day04, solutions);
    input!(day05, solutions);
    input!(day06, optimized);
    input!(day07, solutions);
    input!(day08, solutions);
    input!(day09, solutions);
    input!(day10, solutions);
    input!(day11, optimized);
    input!(day12, solutions);
    input!(day13, optimized);
    input!(day14, optimized);
    input!(day15, optimized);
    input!(day16, optimized);
    input!(day17, solutions);
    input!(day18, solutions);
    input!(day19, optimized);
    input!(day20, solutions);
    input!(day21, solutions);
    input!(day22, solutions);
    input!(day23, optimized);
    input!(day24, optimized);
    input!(day25, solutions);
}
