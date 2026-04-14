// CSES Introductory-Problems Q-16 :: Apple Division - Iterative Bitwise GrayCode Solution
// DateSolved: 14 April 2026
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
    let n: u8 = cin.next();
    let w: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let limit = (1 << n) as u32;
    let mut gray = 0;
    let mut basket_one_sum = 0;
    let total_sum: usize = w.iter().sum();
    let mut min_diff = total_sum;
    for i in 1..limit {
        let bit_to_flip = i.trailing_zeros();
        if gray & (1 << bit_to_flip) == 0 {
            basket_one_sum += w[bit_to_flip as usize];
        } else {
            basket_one_sum -= w[bit_to_flip as usize];
        }
        gray ^= 1 << bit_to_flip;
        min_diff = min_diff.min(basket_one_sum.abs_diff(total_sum - basket_one_sum));
    }
    println!("{}", min_diff);
}