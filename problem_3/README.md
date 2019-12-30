
# Problem 3: Large prime factor

[Problem page](https://projecteuler.net/problem=3)

## Problem Statement

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

## Solution

The only nuance in this solution is its use of the [Fundamental Theorem of Arithmetic](https://en.wikipedia.org/wiki/Fundamental_theorem_of_arithmetic). As such, we can check the value's division against all values of the iterator, without worrying if the value of the iterator is prime. All primes which divide that iterator will have already been divided into the value, and so the iterator will never divide the value unless it is prime.

For example, neither two nor three divide 600851475143, so neither will four or six, as 2*2=4 and 2*3=6. The first prime to divide 600851475143 is, in fact, 71.

## Building

To build with Fortran, you will need the following command:

```shell
gfortran -ffree-form -fno-range-check -o prime_f *.f
```

And this for MSVC:

```shell
cl /EHsc /std:c++17 /Fe:prime_cpp ./*.cpp
```
