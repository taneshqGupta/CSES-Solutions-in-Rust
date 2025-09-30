// CSES Introductory-Problems Q-16 :: Apple Division
// DateSolved: 1 Oct 2025
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
    for i in 0..(1 << n) {
        let mut curr_sum = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                curr_sum += w[j];
            }
        }
        min_diff = min_diff.min(curr_sum.abs_diff(curr_sum.abs_diff(total_sum)));
    }
    println!("{}", min_diff);
}