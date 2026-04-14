// CSES Introductory-Problems Q-13 :: Gray Code
// DateSolved: 14 Apr 2026
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
    let mut gray = 0;
    let limit: u32 = 1 << n;
    println!("{:0width$b}", gray, width = n);
    for i in 1..limit {
        let bit_to_flip = i.trailing_zeros();
        gray ^= 1 << bit_to_flip;
        println!("{:0width$b}", gray, width = n);
    }
}