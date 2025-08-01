// CSES Introductory-Problems Q-6 :: Number Spiral
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
    let t = cin.next();
    
    for _ in 0..t {
        let y: usize = cin.next();
        let x: usize = cin.next();

        let m = x.max(y);
        let ans;

        if m % 2 == 1 {
            if x == m {
                ans = m * m - y + 1;
            } else {
                ans = (m-1).pow(2) + x;
            }
        } else {
            if y == m {
                ans = m * m - x + 1;
            } else {
                ans = (m-1).pow(2) + y;
            }
        }

        println!("{}", ans);
    }
}
