## Hint
Diagonal-based formula: determine which diagonal layer the point belongs to, then calculate its position within that layer based on the spiral's alternating direction pattern.

## Explanation
The solution identifies that the spiral follows a pattern where each diagonal layer (defined by max(x,y)) has a specific starting value and direction. For layer m, the diagonal starts at (m-1)² + 1 and the direction alternates: odd layers fill vertically then horizontally, while even layers fill horizontally then vertically. The algorithm first determines the layer m = max(x,y), then checks if m is odd or even. For odd layers, if x equals m (rightmost column), the value is m² - y + 1 (counting down from the corner); otherwise, it's (m-1)² + x (counting up the bottom row). For even layers, the pattern is reversed: if y equals m (topmost row), the value is m² - x + 1; otherwise, it's (m-1)² + y. This mathematical approach avoids simulation and computes the result in O(1) time per query.
