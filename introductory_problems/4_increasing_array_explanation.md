## Hint
Greedy approach: scan left to right, whenever a decrease is found, increase the current element to match the previous element and add the difference to the operation count.

## Explanation
The solution uses a greedy strategy to make the array non-decreasing with minimum operations. It iterates through the array from left to right, and whenever it encounters an element that is smaller than the previous element (a decrease), it increases the current element to equal the previous element. The number of operations required for this increase is the difference between the previous and current elements, which gets added to the total count. By updating the array element in-place, subsequent comparisons use the corrected value, ensuring the algorithm maintains the non-decreasing property throughout. This greedy approach is optimal because any valid solution must perform at least these operations to fix each decrease, and this strategy uses exactly the minimum required operations.
