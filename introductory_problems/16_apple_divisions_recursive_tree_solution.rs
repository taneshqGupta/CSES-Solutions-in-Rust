// CSES Introductory-Problems Q-16 :: Apple Division -- Recursive Tree Solution
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

    fn recurse(i: usize, curr_sum: usize, total_sum: usize, w: &Vec<usize>, min_diff: &mut usize) {
        if i == w.len() {
            *min_diff = std::cmp::min(*min_diff, curr_sum.abs_diff(curr_sum.abs_diff(total_sum)));
            return;
        }
        recurse(i + 1, curr_sum, total_sum, w, min_diff);
        recurse(i + 1, curr_sum + w[i], total_sum, w, min_diff);
    }
    
    recurse(0, 0, total_sum, &w, &mut min_diff);
    println!("{}", min_diff);;
}
