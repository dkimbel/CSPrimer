# Jug pouring

Uses breadth-first search to solve a 'jug filling' problem, where the inputs are:
- The maximum capacities, in gallons, of some arbitrary number of jugs (all of which start empty).
- The 'goal': how many gallosn we want to end up with in any one of the jugs.

In this case, my program doesn't accept command line arguments -- rather, they're hardcoded in `main`.

Given Jug #1 with max capacity of 3 gallons and Jug #2 with max capacity of 5 gallons, and a goal of
4 gallons, the program outputs:
```
Reached goal after 6 steps!
1. Filled jug #2 by adding 5 gallons.
2. Poured 3 gallons from jug #2 to jug #1, leaving 2 gallons in jug #2 and 3 gallons in jug #1.
3. Emptied jug #1 by dumping out 3 gallons.
4. Poured 2 gallons from jug #2 to jug #1, leaving 0 gallons in jug #2 and 2 gallons in jug #1.
5. Filled jug #2 by adding 5 gallons.
6. Poured 1 gallon from jug #2 to jug #1, leaving 4 gallons in jug #2 and 3 gallons in jug #1.
```
