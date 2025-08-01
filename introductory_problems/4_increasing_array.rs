// CSES Introductory-Problems Q-4 :: Increasing Array
// DateSolved: 27 Jun 2025
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
    let mut aa: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    
    let mut ans = 0;

    for i in 1..n {
        if aa[i] < aa[i-1] {
            ans += aa[i-1] - aa[i];
            aa[i] = aa[i-1];
        }
    }

    println!("{}", ans);
}