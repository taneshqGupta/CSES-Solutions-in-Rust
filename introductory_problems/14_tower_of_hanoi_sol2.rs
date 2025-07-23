// CSES ProblemSet Q-14 :: Tower of Hanoi
// DateSolved: 18 Jul 2025
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

fn tower(n: u32, a: u8, b: u8, c: u8) {
    if n > 0 {
        tower(n - 1, a, c, b);
        println!("{} {}", a, c);
        tower(n - 1, b, a, c);
    }
}

fn main() {
    let mut cin = Scanner::new();
    let n = cin.next();
    println!("{}", u16::pow(2, n) - 1);
    tower(n, 1, 2, 3);
}