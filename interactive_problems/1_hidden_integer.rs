// CSES Interactive-Problems Q-1 :: Hidden Integer
// DateSolved: 21 Jul 2025
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
    let mut minp = 1;
    let mut maxp = 1e9 as usize;
    while maxp != minp {
        let guess = minp + (maxp - minp) / 2;
        println!("? {}", guess);
        let response: String = cin.next();
        if response == "YES" {
            minp = guess + 1;
        }
        else {
            maxp = guess;
        }
    }
    println!("! {}", minp);
}
