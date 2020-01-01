
# Problem 7: 10001st prime

[Problem page](https://projecteuler.net/problem=7)

## Problem Statement

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10001st prime number?

## Solution

By the [prime number theorem](https://en.wikipedia.org/wiki/Prime_number_theorem#Approximations_for_the_nth_prime_number), there is a known upper bound on the `n`th prime if `n` is greater than six. The programme then enumerates all primes up to this upper bound using the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes). Thence, the 10001st prime can be found.
