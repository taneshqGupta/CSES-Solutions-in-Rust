// CSES Introductory-Problems Q-2 :: Missing Number
// DateSolved: 24 Jun 2025
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

    const N: usize = 3e5 as usize;
    let mut aa: [usize; N] = [0; N];
    let mut ans = 0;

    for _ in 0..n-1 {
        let curr: usize = cin.next();
        aa[curr] += 1;
    }    

    for i in 1..=n {
        if aa[i] == 0 { 
            ans = i; 
        }
    }    
    
    println!("{}", ans);
}