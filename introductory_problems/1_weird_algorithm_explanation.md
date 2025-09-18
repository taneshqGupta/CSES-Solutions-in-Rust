## Hint
Direct simulation of the Collatz conjecture: repeatedly apply the rule (divide by 2 if even, multiply by 3 and add 1 if odd) until reaching 1.

## Explanation
The solution implements a straightforward simulation of the weird algorithm (Collatz conjecture). It reads the initial number n and enters a loop that continues until n becomes 1. In each iteration, it prints the current value of n, then applies the transformation rule: if n is even, divide by 2; if n is odd, multiply by 3 and add 1. The algorithm terminates when n reaches 1, at which point it prints the final 1. This approach directly follows the problem statement without any mathematical optimizations or shortcuts, making it simple and reliable for the given constraints.
