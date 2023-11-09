# Day 16: Proboscidea Volcanium

## Summary

We've encountered a cave network where each room has a valve in it.
Each valve has a flow rate N, and a number of tunnels leading to other valves.
At every minute, we may open a valve or move to an adjacent valve.
A valve will release N pressure for every minute after it has been opened.
Our goal is to release as much pressure as possible.

## Part 1

How much pressure can you release in 30 minutes?

### Abstracting the problem

TODO: Is there a tighter classification of this problem?

This exercise can be classified as a
[combinatorial optimization problem](https://en.wikipedia.org/wiki/Combinatorial_optimization).
We have a number of turns, a "game state",
a sequence of available actions based on the state,
and each action can modify the state and/or increase the score.
The challenge stems from the vast space of possibilities and the fact
that the greedy choice isn't always the best one.

We will solve the problem using a state-space search of the decision tree,
employing various methods to help us:

- Pruning bad branches using branch-and-bound.
- Predicting the payout of actions up-front.

TODO: Rephrase

- Searching through "meta-actions",
  sequences of actions that have a meaningful result.

### Pre-processing

If we were to perform a DFS of the decision tree,
we would be searching through a lot of obviously useless decisions.
Each valve may lie on the path to another,
so we cannot mark the visited valves and ignore them.
In the first place, only 15 of the 61 valves have a non-zero flow rate.

To drastically reduce the search space,
we create a complete weighted graph of the non-zero valves,
each edge corresponding to the shortest path between the connected valves.

In this graph, going across a single edge to a node
corresponds to taking the shortest path to that valve and opening it.
There is no possibility of aimlessly walking around,
and we can track which valves have already been opened.

### Branch and bound

The most important factor to improving performance is to reduce the amount of states we search.
To prune the search space,
we'll use a [branch and bound](https://en.wikipedia.org/wiki/Branch_and_bound) method.
It's a method of pruning a state space search,
and it more or less boils down to:

- Establish a heuristic to search through better prospective paths first.
- Keep track of a lower bound for the so-far best solution.
- Skip all branches whose upper bound isn't greater than the current lower bound.

For the __heuristic__, we prioritize opening whichever valve has the greatest flow.
This could be further specified to
whichever valve provides the greatest immediate score after travel.

The __lower bound__ is the best score that we have encountered so far.

The __upper bound__ is calculated as an optimistic best final score.
If we reduced all distances between valves to the shortest of all distances,
the best score could be obtained by always opening the valve remaining with highest flow.

Pruning our branches this way,
we only visit __246__ states before we know the absolute best score.

## Part 2

An elephant decides to help out.
You may each perform an action at every minute.
Release as much pressure as possible within only 26 minutes.

### Observations

Our goal this time is to find the two non-overlapping solutions (complements)
that achieve the greatest total score.

### Incorrect attempt

A popular way to solve the second part was to solve for one person first,
then solve for the elephant using the remaining unvisited valves.
_This appproach is incorrect._
There is no guarantee that the best solution and its best complement will yield the highest total.

### Method

1. Find the optimal single path, P.
2. Find the best complement to P, C.
3. Traverse all branches that >= C.
   Store the best score for each set of visited nodes.
4. Out of all non-zero sets, find the best complementing pair.
