# Day 6

## Skip ahead, check from the back

The naive solution is to simply go through every N-character window
and find the first where all characters are unique.
But once we find two characters that are unique,
we know that every window containing both of these wont be a solution.

This we can exploit.
We build our set from the last letter in the window to the first.
As soon as we encounter a letter that is already contained in the set,
we move the start of our window to the position right after this.
This lets us skip the majority of windows.

## Bitsets

This is the first time where we'll be using bitsets.
If we want to represent a set with a reasonably small number of possible elements,
we can use an unsigned integer to do so.
Each bit corresponds to the inclusion of a specific element in that set.
The union can then be computed using bitwise `OR`,
the intersection using bitwise `AND`, the difference using XOR, etc.

In this case, we can map the letters `[a-z]` to the bit positions from `0` to `25`.
This is much faster than using a `HashSet`,
taking only a few instructions to query, add, and remove elements.

