# Day 5

Another just-follow-the-instructions day.
We will hold the state in a `Vec<Vec<char>>`,
and use pop/push operations to move around our boxes.

We do run into a small issue regarding lifetimes.
In part 2, we want to take a slice off the top of one stack and write it onto another.
This operation could be performed fast if we had mutable access to both the source and destination.
The compiler wont lend us mutable references to two indices at once, however,
as it cannot statically know that we wont borrow the same element twice.
Actually, we cannot be _sure_ either.
Maybe some inputs require moving from a stack to itself?
To overcome this,
we move the crates to a temporary holding buffer
before placing them back onto the destination stack.
