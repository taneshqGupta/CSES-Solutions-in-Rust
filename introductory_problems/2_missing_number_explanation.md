## Hint
Use frequency counting: create an array to mark which numbers are present, then scan from 1 to n to find the unmarked position.

## Explanation
The solution uses a frequency array approach to find the missing number. It creates an array `aa` of size 300,000 initialized to zeros, then reads n-1 numbers and increments the count at each number's index in the array. After processing all input numbers, it iterates through positions 1 to n in the frequency array and finds the position with count 0, which corresponds to the missing number. This approach has O(n) time complexity for both marking present numbers and finding the missing one, making it efficient and straightforward. The space complexity is O(max_value) but since the constraint allows up to 2×10^5, a fixed-size array works well.
