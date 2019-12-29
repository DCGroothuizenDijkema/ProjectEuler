
# Problem 5: Smallest multiple

[Problem page](https://projecteuler.net/problem=5)

## Problem Statement

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

## Solution

There are several ways to solve this problem programatically and mathematically.

We first note that the lowest common multiples (lcm) of all numbers from 1 to 10, which is 2520, must be a divisor of the lcm of all numbers from 1 to 20, and must the latter must be at least the former. Therefore, we can begin our loop at 2520 and increment by 2520. Furthermore, we do not need to check if every number from 1 to 20 divides into the iterator's current value. 1 always will, and if we know, for example, that 20 divides it, we do not need to know if 2 nor 5 do, as 20=2*2*5. Therefore, we need only check if primes from 2 to 20 divide the number, but these primes will have to be raised to the highest power with which they have in the prime factorisation of all numbers from 2 to 20.

This leads us to realising how we can solve this problem mathematically. The lcm of all numbers from 1 to 20 will have some prime factorisation, for any number to be a factor of this lcm means that it is either one of those primes are thereby also factorised. In other words, we need to find the smallest number whose prime factorisation can be used to construct every number from 2 to 20. Therefore, we simply need to enumerate the prime factorisation of every number from 2 to 20. For each prime factor, take the occurence which has the largest exponent, and multiply all results together. This will give the lcm.
