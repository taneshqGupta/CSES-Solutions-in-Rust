// CSES Sorting & Searching Q-8 :: Maximum Subarray Sum
// DateSolved: 13 April 2026
// SolvedBy: taneshqGupta

#[derive(Default, Debug)]
struct Scanner(Vec<String>);
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(c) = self.0.pop() {
                return c.parse().ok().unwrap();
            }
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            self.0 = s.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    let a: Vec<isize> = (0..n).map(|_| cin.next()).collect();
    /*
        We use the Kadane's Algorithm
        Complexity is O(n)
        A single pass is enough
        We scan the list from left to right.

        In the j'th step, we
        compute the subarray with the
        largest sum ending at index j.
        This is either the element itself
        or the element at the end of the
        last element's best subarray.

        Let us assume that the value at 
        position i is x and we want to 
        calculate the maximum subarray sum 
        that ends at position i. 
        We can do this efficiently when 
        we know the previous maximum subarray sum 
        that ends at position i-1: 
        either we add x to the previous sum 
        or then just create a sum using x.

        Basically, if the sum ever goes negative,
        just start a new subarray.

        The maximum of all these maximums
        is the answer.
    */

    let mut best_ending_here = a[0];
    let mut best = a[0];

    for i in 1..n {
        best_ending_here = a[i].max(best_ending_here + a[i]);
        best = best.max(best_ending_here);
    }

    println!("{}", best);
}