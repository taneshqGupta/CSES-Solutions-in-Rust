## Hint
Recursive binary reflection: generate Gray code by taking the previous sequence, appending 0 to each, then appending the reversed sequence with 1s appended.

## Explanation
While writing gray codes, I noticed a pattern. Suppose you start by considering 0 to be the 1st element, 1 to be the second, and so on. By this numbering, whenever you hit a number that of the form 2^n, that is, when you reach the 4th, 8th, 16th, 32nd, elements, etc. When you reach such elements, the next gray code of the next 2^n elements is just created by adding one extra bit to the leftmost of our element, and then writing the last 2^n elements in reverse. 
Eg. 0, 1  -> 11, 10  --> 110, 111, 101, 100 ---> 1100, 1101, 1111, 1110, 1010, 1011, 1001, 1000 ----> etc...