// CSES Introductory-Problems Q-13 :: Gray Code
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

fn gray(n: u8) -> Vec<Vec<bool>> {
    if n == 1 {
        return vec![vec![false], vec![true]];
    }
    let aa = gray(n - 1);
    aa.iter()
        .map(|x| [&x[..], &[false]].concat())
        .chain(aa.iter().rev().map(|x| [&x[..], &[true]].concat()))
        .collect()
}

fn main() {
    let mut cin = Scanner::default();
    let aa = gray(cin.next());
    for a in aa {
        for &c in a.iter().rev() {
            print!("{}", if c { 1 } else { 0 });
        }
        println!();
    }
}
