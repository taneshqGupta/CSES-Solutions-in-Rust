// CSES ProblemSet Q-5 :: Permutations
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

    match n {
        1 => { println!("1"); }
        2 | 3 => { println!("NO SOLUTION"); }
        4 => { println!("3 1 4 2"); }
        _ => {
            let ans: Vec<usize> = (2..=n).step_by(2).chain((1..=n).step_by(2)).collect();
            for c in ans {
                print!("{} ", c);
            }
            println!();
        }
    }
}
