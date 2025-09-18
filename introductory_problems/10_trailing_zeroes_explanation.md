## Hint
Count factors of 5 in n! using Legendre's formula: sum ⌊n/5⌋ + ⌊n/25⌋ + ⌊n/125⌋ + ... since trailing zeros come from factors of 10 = 2×5.

## Explanation
The solution counts trailing zeros in n! by counting the number of times 10 divides n!. Since 10 = 2×5 and there are always more factors of 2 than 5 in any factorial, the number of trailing zeros equals the number of times 5 appears as a factor in n!. Using Legendre's formula, it counts multiples of 5 (contributing one factor each), multiples of 25 (contributing an additional factor), multiples of 125 (contributing yet another factor), and so on. The algorithm iterates through powers of 5 (5, 25, 125, ...) and for each power i, adds ⌊n/i⌋ to the count, representing how many numbers from 1 to n are divisible by that power of 5. This continues until the power exceeds n, giving the total count of factor-5 contributions and thus the number of trailing zeros.
