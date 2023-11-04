# The Rules

There isn't any agreed-upon standard for benchmarking AoC solutions,
so we'll have to define some "ground rules".
Our rules will be just one out of many valid interpretations.

We define a solution as a function:

- Input: An ASCII string with a known size.
- Output: Two printable solution values, one for each part.

We may assume that the input is valid ASCII,
as backed up by all previous inputs.
We don't measure the time it takes to load the input into memory,
as that says more about our internet connection or SSD than our solutions.
We also allow ourselves to know the size of the input,
as there isn't any good reason we _shouldn't_ know this.

An output value should be trivially printable.
Naturally, strings and integers are allowed.
Slightly more controversially, printable grids are also allowed.

## Input "protocol"

TODO: Make it clear that "all inputs" means all dayXX inputs across users.

The input isn't formally specified,
which can be a bit of a slippery slope
when it comes to optimization.
We do wish to take advantage of patterns in the input data,
but we don't want to effectively hardcode our solutions.
As a middle ground, I suggest that the input is limited by two kinds of rules:

1. Rules that are explicitly described in the text.
2. Rules that are implicitly observed across all inputs.

Some examples of the latter are grid dimensions, entity counts, etc.
In order for a solution to be considered correct,
it should function on any input that follows all rules of both types.
In other words, it shouldn't be possible to construct an input which follows all rules,
but breaks the solution.

## Benchmarking

Since the solutions run in microseconds,
measuring the time elapsed for a single run isn't a reliable benchmark.
Instead, we rely on Criterion to perform a more accurate assessment.
Criterion measures execution time by running the target a lot of times back-to-back.
After a ~3 second warmup,
the solution is run for ~5 seconds,
and the average execution time is computed from the results.

TODO: Mention input cycling if performed.
