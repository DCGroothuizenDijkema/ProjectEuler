
# Problem 14: Longest Collatz sequence

[Problem page](https://projecteuler.net/problem=14)

## Problem Statement

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.

## Solution

The only practical feature to enhance solution time for this problem is to note that if the Collatz sequence falls on a number whose sequence has already been calculated, no more computation needs doing. The function should instead return the current iteration count summed with the iteration count of the other number.

The solution could also keep track of the Collatz sequence and back track along it to fill in the iteration count of all numbers in that sequence. However, testing found that this was slower for this given problem. Perhaps it would be faster if the upper limit exceed one million.
