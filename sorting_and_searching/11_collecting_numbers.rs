// CSES Sorting & Searching Q-11 :: Collecting Numbers
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
    let n: usize = cin.next();
    let mut x: Vec<usize> = vec![0; n];
    let mut position: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        x[i] = cin.next();
        position[x[i]] = i;
    }

    // a new round is required if
    // and only if position of i + 1
    // is less than position of i

    let mut rounds = 1;

    for i in 2..=n {
        if position[i] < position[i - 1] {
            rounds += 1;
        }
    }

    println!("{}", rounds);
}