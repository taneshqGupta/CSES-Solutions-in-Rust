## Hint
Categorize chessboard squares by position type (corners, edges, center) since each type has different attack patterns, then use combinatorial counting: for each square type, multiply count by (total squares - squares it attacks).

## Explanation
The solution divides the k×k chessboard into 7 categories based on how many squares each position can attack. Corner squares (x1,x2,x3,x4) each attack 3-5 squares depending on their specific corner, edge squares attack 4-7 squares (with two subcategories e1,e2 for different edge positions), and center squares attack exactly 8 squares. For each category, it calculates the number of non-attacking placements as: (number of squares in category) × (total board squares - number of squares that category attacks). The final answer sums contributions from all categories and divides by 2 since each knight pair is counted twice (once for each knight being the "first" knight). Cases k=1,2,3 are hardcoded.
