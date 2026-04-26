// CSES Sorting & Searching Q-25 :: Sum of 3 Values
// DateSolved: 26 April 2026
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
    let x: usize = cin.next();
    let mut a: Vec<(usize, usize)> = (0..n).map(|i| (cin.next(), i + 1)).collect();
    a.sort_unstable();

    let mut ans: Option<(usize, usize, usize)> = None;

    'outer: for i in 0..n {
        if a[i].0 >= x {
            break 'outer;
        }
        let target = x - a[i].0;
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sumx = a[left].0 + a[right].0;
            if sumx == target {
                ans = Some((a[i].1, a[left].1, a[right].1));
                break 'outer;
            } else if sumx < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    if let Some((a, b, c)) = ans {
        println!("{} {} {}", a, b, c);
    } else {
        println!("IMPOSSIBLE");
    }
}