pub fn run(input: &str) -> (u32, u32) {
    (0, 0)
}

// So the primary operation would have to take less than 1ms/1116.
// Somewhere around 800 nanoseconds.

// The array would be a bit grid represented by [(u128, u128); 216].
// Or a [u32x8; 216];
// It'd be good to properly bound the grid.
// If a row could fit in a u128, the speed would be doubled.

// 1. Make horizontal and vertical convolutions. (4x)
// 2. Find elves that should move (4x).
// 3. Decide on movement directions (4x)
// 4. Cancel on conflicts (4x)
// 5. Perform movement (5x?)

// This approach comes out to ~21 shift-and-combine operations for a round, maybe more.
// So each shift-and combine gets 40 nanoseconds.
// Is that possible?
// Not for an array...

// Would it be possible to spread out the work to multiple cores?
// You would need synchronization across block boundaries for each step.
// If the grid were huge, that'd be fine, but the overhead would be too great.

// The options:
// 1. Find a way to do less computations.
// 2. Loop unrolling.
// 3. SIMD.
// 4. GPU optimization.
//    Biggest question: Is the overhead of talking to the GPU a problem?
//    My guess would be no.
