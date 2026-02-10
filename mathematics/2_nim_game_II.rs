// CSES Mathematics Q-2 :: Nim Game II
// DateSolved: 10 Feb 2026
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
    for _ in 0..cin.next() {
        let n: usize = cin.next();
        let mut xor_sum = 0;
        for _ in 0..n {
            let mut temp: usize = cin.next();
            temp %= 4;
            xor_sum ^= temp;
        }
        if xor_sum == 0 {
            println!("second");
        } else {
            println!("first");
        }
    }
}
