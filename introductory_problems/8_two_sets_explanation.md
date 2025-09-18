## Hint
Modular arithmetic check and symmetric pairing: if n%4 is 1 or 2, no solution exists; otherwise use symmetric pairing strategies for n%4=0 and n%4=3 cases.

## Explanation
The solution first checks if a valid partition exists using modular arithmetic. Since the sum 1+2+...+n = n(n+1)/2 must be even for equal division, this happens only when n%4 = 0 or n%4 = 3. For n%4 = 0, it uses symmetric pairing: the first set gets pairs (1,n), (2,n-1), ..., (n/4, 3n/4+1) and the second set gets the remaining middle pairs. This creates two sets of size n/2 each with equal sums. For n%4 = 3, it uses a different pairing strategy: pairs consecutive numbers (1,2), (3,4), etc., alternating which set they go to, then puts the final element n in the second set. The key insight is that both strategies maintain the sum balance: symmetric pairs in the n%4=0 case have identical sums, while the alternating pattern in the n%4=3 case balances out with the final singleton element.
