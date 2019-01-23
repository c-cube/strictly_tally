# Strictly Tally

A rust program to rank dancers, following
[this excellent blogpost](https://jazzmonkey.wordpress.com/2013/04/19/6-steps-to-understanding-relative-placement/).

It eats a .csv file and outputs ~~a panic stacktrace~~ the list of candidates, ranked.

**Note**: it doesn't implement the "showdown" version of type-2 ties. Instead
it reports ties in its output for a supreme judge to cut.

## Example

```
$ ./tally.sh tests/test1.csv

=== Sheet ===
name            j1  j2  j3  j4  j5 
Jim Bob           1   4   2   2   5
Freddy Lou        3   1   3   4   2
Mary Sue          4   3   5   3   1
Bobby Joe         2   5   1   5   3
Candy Jane        5   2   4   1   4
=============

## ranking competitors (majority at 3)

### ranking competitors ["Jim Bob", "Freddy Lou", "Mary Sue", "Bobby Joe", "Candy Jane"] (lvl 1, majority at 3)

## computing 1-placements
1-placements: [1, 1, 1, 1, 1]

## computing 2-placements
2-placements: [3, 2, 1, 2, 2]
winner for this round (by majority): "Jim Bob"

## computing 2-placements
2-placements: [3, 2, 1, 2, 2]

## computing 3-placements
3-placements: [3, 4, 3, 3, 2]
several candidates: [("Freddy Lou", 4), ("Mary Sue", 3), ("Bobby Joe", 3)]
winner for this round (biggest majority of 4): "Freddy Lou"

## computing 3-placements
3-placements: [3, 4, 3, 3, 2]
several candidates: [("Mary Sue", 3), ("Bobby Joe", 3)]

### computing 3-Σ-placements
3-Σ-placements: [(CompetitorID(2), 7), (CompetitorID(3), 6)]
several candidates have majority, smallest sum is 6
winner for this round (with smallest sum 6): "Bobby Joe"

## computing 3-placements
3-placements: [3, 4, 3, 3, 2]
winner for this round (by majority): "Mary Sue"

## computing 3-placements
3-placements: [3, 4, 3, 3, 2]

## computing 4-placements
4-placements: [4, 5, 4, 3, 4]
winner for this round (by majority): "Candy Jane"

############
ranked: ["Jim Bob", "Freddy Lou", "Bobby Joe", "Mary Sue", "Candy Jane"]

```
