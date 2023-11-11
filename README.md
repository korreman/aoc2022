Blazingly fast solutions to all parts of AOC 2022.

Inputs and answers should be placed in `data/<PROFILE_NAME>/`,
where `<PROFILE_NAME>` can be any name you desire.

Inputs should be placed in `data/<PROFILE_NAME>/inputs/` as `day01.txt`, `day02.txt`, etc.
For integration testing, solutions should be placed in `data/<PROFILE_NAME>/solutions/`
with the same file naming scheme.
A solution file should contain only the solution strings for the first and second part,
separated two newlines for example:

    23987234

    43742

Running the solution normally with `cargo run --release` will solve for all profiles and inputs.
Integration tests can be run with `cargo test inputs`.
Benchmarks can be run with `cargo bench` (uses `criterion`).

Having multiple sets of inputs is supported by placing them in
`data/profile_a`, `data/profile_b`, etc.
This is useful for testing the robustness of your solution,
but remember not to share your inputs with the world!
