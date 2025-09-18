## Hint
Binary search on answer space: repeatedly query middle values and narrow the search range based on YES/NO responses until the range contains only one number.

## Explanation
The solution uses binary search to find the hidden integer within the range [1, 10^9]. It maintains two pointers, minp and maxp, representing the current search bounds. In each iteration, it queries the middle value of the current range. If the response is "YES" (meaning the hidden number is greater than the guess), it updates minp to guess+1, effectively searching the upper half. If the response is "NO" (meaning the hidden number is less than or equal to the guess), it updates maxp to guess, searching the lower half including the guess. This process continues until minp equals maxp, at which point the hidden number is found. The binary search guarantees finding the answer in at most ⌈log₂(10^9)⌉ ≈ 30 queries, making it efficient within the query limit.
