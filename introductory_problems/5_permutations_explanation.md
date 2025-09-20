## Hint
Even-odd interleaving: place all even numbers first, then all odd numbers to avoid consecutive numbers in the permutation (except special cases n=2,3,4).

## Explanation
The solution constructs a beautiful permutation by using an even-odd pattern to avoid adjacent numbers. For n=1, it trivially outputs "1". For n=2 and n=3, no solution exists since any arrangement will have consecutive numbers adjacent. For n≥5, the algorithm outputs all even numbers in ascending order (2,4,6,...) followed by all odd numbers in ascending order (1,3,5,...). This ensures that no two consecutive integers appear next to each other in the permutation because the difference between consecutive even numbers is 2, the difference between consecutive odd numbers is 2, and the transition from the largest even to smallest odd creates a gap that avoids consecutive integers.
