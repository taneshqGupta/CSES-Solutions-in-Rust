## Hint
Utilise the fact that there are exactly 8 queens. 
Construct 'paths' one at a time using DFS, instead of counting by PNC.

## Explaination
In every valid path, every row must contain exactly one queen, no less, no more. Same goes for columns. My solution implements a DFS considering rows as levels. Same solution can be achieved by considering columns as levels.
You must use DFS, as in DFS, we only consider one path at a time. Whenever doing DFS, consider:
1. What are you using as tree levels? We use rows, this is the variable that we will use to backtrack.
2. What is the state that defines each path, here the state is the array denoting columns taken, left diagonals taken, and right diagonals taken

Now how do we calculate in constant time which element belongs to which left diagonal and which right diagonal?
For this we must study some left diagonals and right diagonals indices. When we do so, we realise that for each left diagonal, the indices sum is constant, furthermore, the sum is ranging from 0 to 14. Sum 0 happens in the single point diagonal of the point (0, 0). Sum 14 happens in the single point diagonal of (7, 7). Sum 7 happens in the biggest center diagonal that extends from point (0, 7) to (7, 0).
Also, for each right diagonal, when we study them, we will realise that for each right diagonal, the difference of the indices are constant. Furthermore, the differences are ranging from -7 to +7. Difference -7 happen in the single point diagonal of (0, 7). Difference +7 happen in the single point diagonal of (7, 0). Difference 0 happens in the center biggest diagonal that extends from (1, 1) to (7, 7).
Now for every single element, we can check in constant time which left diagonal it belongs to by adding its row and column indices. We can check which right diagonal it belongs by taking the difference of its row and column indices. 
So we can store the state of the current path by just storing 15 booleans for left diagonals, 15 booleans for right diagonal and 8 booleans for columns taken. And on each new column in every row, we choose to call further DFS on it if it does not belong to any taken diagonal or column and if it isnt '*'.
