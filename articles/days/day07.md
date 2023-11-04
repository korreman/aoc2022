# Day 7

For this day, we make one important observation.
The traversal described in the input is performed depth-first.
We treat this as one of the __implicit__ rules for this input,
which we are allowed to take advantage of.

With that, the traversal becomes a typical tree description.
It could easily be converted to an S-expression:

- `$ cd name` and `$ ls` corresponds `(name `.
- `$ cd ..` corresponds to `)`.
- `SIZE NAME` corresponds to `(SIZE NAME)`.

And et voilá! An S-expression tree!
By treating the input as a description of the tree,
we can traverse it using only a stack.

Now the solution is pretty simple:

- If a new node is opened, push another element to the stack.
- If a file is listed, add its size to the top element of the stack.
- If a node is closed:
    - Add its size to the part 1 solution if `< 100000`.
    - Pop it from the stack and add it to the element below it.

To solve part 2, we generate a list of all directory sizes along the way.
Then we filter out all that are too small to free enough space,
and take the minimum of the rest.

We also save some time by completely ignoring the names of directories and files.
No need to track them, we don't need them!
