## Hint
Binary exponentiation: compute 2^n mod (10^9 + 7) efficiently using repeated squaring to avoid overflow and achieve O(log n) complexity.

## Explanation
The solution computes 2^n mod (10^9 + 7) using binary exponentiation (fast exponentiation). Instead of multiplying 2 by itself n times which would take O(n) time and cause overflow, it uses the binary representation of n to build the result efficiently. The algorithm maintains two variables: `ans` (current result) and `base` (current power of 2). In each iteration, if the current bit of n is 1, it multiplies `ans` by the current `base`. Then it squares the `base` and shifts n right by one bit. This process continues until n becomes 0. The key insight is that any exponent can be expressed as a sum of powers of 2 (its binary representation), so 2^n = 2^(sum of powers of 2) = product of corresponding powers. The modular arithmetic prevents overflow while maintaining correctness.
