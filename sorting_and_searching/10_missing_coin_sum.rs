// CSES Sorting & Searching Q-10 :: Missing Coin Sum
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
    let mut a: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    a.sort_unstable();
    
    // We maintain a range [1, S]
    // This is the correct range
    // for array till i'th element
    // here. Then we determine
    // whether current new element
    // produces gap on being introduced or not

    let mut smax = 0;
    let mut result = None;
    for i in 0..n {
        if a[i] > smax + 1 {
            result = Some(smax + 1);
            break;
        }
        smax = a[i] + smax;
    }
    if let Some(ans) = result {
        println!("{}", ans);
    } else {
        println!("{}", smax + 1);
    }
}