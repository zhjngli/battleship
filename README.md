# battleship

Given an N x N battleship board, and a hidden 3 x 1 battleship that is placed randomly (a uniform chance of any legal position) on the board (horizontal or vertical, but not diagonal), how can you minimize the number of guesses it takes before you find the battleship?

```
Total runs: 1000
Average guesses for Loop: 28.163
Average guesses for Random: 16.963
Average guesses for MaxReduceRemaining: 10.046
```

Each strategy implemented returns the number of guesses it took before reaching the first hit. The technique for finding the battleship after the first hit is the same for each strategy, so I just focus on the first hit.

## strategies

### loop
The most naive approach is to loop through each cell in the board and check if it's a hit. This is a deterministic strategy, so we can calculate the expected value of guesses by counting the number of guesses it would take to hit every possible battleship. For `N = 8` and a battleship of size `3` this comes out to `28`.

### random
Randomly guessing cells is (un)suprisingly better than the loop strategy. You don't need to guess the cell immediately next to a previously checked cell, since it's less likely that that cell will hit a battleship. Random guessing inadvertently takes advantage of this fact. This is like pulling objects out of a bag where the total number of objects is `N^2` and the number of desired objects is `3`. What is the expected number of pulls it takes to pull a desired object? For `N = 8` this is about `16.25`.

### max reduce remaining
The basic idea of this strategy is to reduce the number of remaining possible battleship placements as much as possible with each guess:

1. First create the set of all potential battleship placements
1. Inside a loop
    1. Create a map with key location, and value a list of any potential battleship that overlaps with that location
    1. Find the location with the most battleships that overlap with it
    1. Assuming the location is not a hit, remove those battleships from the set of potential battleships
1. If the location was a hit, break out of the loop and return the number of guesses

The initial guesses should always reduce the number of possible remaining battleships by `3 * 2 = 6`. Each location in the center of the board will always overlap with `6` possible battleships. From there the algorithm continues to reduce its search space until it finds a hit. Don't know how to find the expected value here but it seems like it's around `10`.
