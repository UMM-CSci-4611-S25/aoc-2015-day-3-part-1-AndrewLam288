[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/WEKFfWR3)
# AoC-2015-Day-03

A simple Rust exercise based on an Advent of Code problem from 2015, [Day 3: Perfectly Spherical Houses in a Vacuum](https://adventofcode.com/2015/day/3).

We'll go through part 1 together in class, and then have small groups sort out part 2 on your own. Below are the problem statements, followed by a sketch of how we solved part 1 together.

## Problem statement

The following problem statements are taken directly from [the Advent of Code problem statements](https://adventofcode.com/2015/day/3).
Typically part 2 of Advent of Code problems is not visible until you've completed part 1, so I'm cheating slightly by
sharing the problem statement of part 2 with everyone.

### Part 1

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then
an elf at the North Pole calls him via radio and tells him where to move next.
Moves are always exactly one house to the north (`^`), south (`v`), east (`>`), or west (`<`).
After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so
his directions are a little off, and Santa ends up visiting some houses more than
once. How many houses receive at least one present?

For example:

- `>` delivers presents to 2 houses: one at the starting location, and one to the east.
- `^>v<` delivers presents to 4 houses in a square, including twice to the house at
  his starting/ending location.
- `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.

### Part 2

The next year, to speed up the process, Santa creates a robot version of himself,
Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the
same starting house), then take turns moving based on instructions from the elf,
who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

- `^v` delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
- `^>v<` now delivers presents to 3 houses, and Santa and Robo-Santa end up back where
  they started.
- `^v^v^v^v^v` now delivers presents to 11 houses, with Santa going one direction and
  Robo-Santa going the other.

---

## Solution sketch for part 1

I've provided unit tests that will cover most of the necessary logic, and we'll let those
drive our development. Below we'll mark key decisions that need to be made as we go.
