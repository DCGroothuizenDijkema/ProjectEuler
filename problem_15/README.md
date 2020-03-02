
# Problem 15: Lattice paths

[Problem page](https://projecteuler.net/problem=15)

## Problem Statement

Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

_(see problem page for example grid)_

How many such routes are there through a 20×20 grid?

## Solution

In enumerating basic examples and determining how many paths there are, both to the end and to each point, [Pascal's triangle](https://en.wikipedia.org/wiki/Pascal%27s_triangle) nicely emerges. The number of paths to the end proves to be the central number of even rows of Pascal's triangle. As the *n*th row and *k*th column of Pascal's triangle takes the form of *n* choose *k*, to determine the number of paths to the end of a *n*-by-*n* grid the programme needed to calculate *2n* choose *n*. This can be done with in [a multiplicative way](https://en.wikipedia.org/wiki/Binomial_coefficient#Multiplicative_formula), as oppose to [the common recursive way](https://en.wikipedia.org/wiki/Binomial_coefficient#Recursive_formula) or [the factorial way](https://en.wikipedia.org/wiki/Binomial_coefficient#Factorial_formula).
