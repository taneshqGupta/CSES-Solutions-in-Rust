## Hint
Mathematical constraint verification: check if (a+b) is divisible by 3 and if min(a,b) ≥ (a+b)/3, since each move removes exactly 3 coins total.

## Explanation
The solution verifies if two coin piles can be emptied using the allowed moves (remove 2 from one pile and 1 from another, or remove 1 from one and 2 from another). Each move removes exactly 3 coins total, so the sum a+b must be divisible by 3 for complete emptying. The number of required moves is (a+b)/3. For feasibility, the algorithm checks that the smaller pile has enough coins to participate in all moves, meaning min(a,b) ≥ (a+b)/3. This constraint ensures that even if the smaller pile contributes the minimum (1 coin per move), it won't be exhausted before all moves are completed. If both conditions are satisfied, the piles can be emptied; otherwise, it's impossible. The solution efficiently checks these mathematical constraints without simulating the actual move sequence.
