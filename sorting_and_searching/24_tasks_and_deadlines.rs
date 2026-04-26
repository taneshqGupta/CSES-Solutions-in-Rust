// CSES Sorting & Searching Q-24 :: Tasks & Deadlines
// DateSolved: 26 April 2026
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
    let mut tasks: Vec<(isize, isize)> = (0..n).map(|_| (cin.next(), cin.next())).collect();
    tasks.sort_unstable();
    let mut time: isize = 0;
    let mut reward = 0;
    for (duration, deadline) in tasks {
        time += duration;
        reward += deadline;
        reward -= time;
    }
    println!("{}", reward);
}