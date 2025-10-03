// CSES Introductory-Problems Q-16 :: Apple Division - Iterative Bitwise GrayCode Solution
// DateSolved: 3 Oct 2025
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
    let w: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let total_sum: usize = w.iter().sum();
    let mut min_diff = 1e19 as usize;
    let mut curr_sum: usize = 0;
    let mut prev_gray: usize = 0;
    for i in 1..(1 << n) {
        let gray_i = i ^ (i >> 1);
        let mask: usize = prev_gray ^ gray_i;
        let changex = mask.trailing_zeros() as usize;
        if (gray_i >> changex) % 2 == 1 {
            curr_sum += w[changex];
        } else {
            curr_sum -= w[changex];
        }
        min_diff = min_diff.min(curr_sum.abs_diff(curr_sum.abs_diff(total_sum)));
        prev_gray = gray_i;
    }
    println!("{}", min_diff);
}