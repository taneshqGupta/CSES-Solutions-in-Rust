## Hint
Single pass with running count: track consecutive identical characters using dynamic programming, storing the length of repetition ending at each position.

## Explanation
The solution processes the DNA string character by character, maintaining an array where `aa[i]` stores the length of the consecutive sequence of identical characters ending at position i (excluding the character itself). For each position i starting from 1, if the current character matches the previous character, it extends the sequence by setting `aa[i] = aa[i-1] + 1`; otherwise, `aa[i]` remains 0 (indicating no extension). After processing the entire string, the maximum value in the array represents the longest extension length, so adding 1 gives the actual length of the longest repetition. This approach efficiently computes the result in O(n) time and O(n) space using dynamic programming principles.
