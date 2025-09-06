// CSES Interactive-Problems Q-5 :: Colored Chairs
// DateSolved: 6 Sep 2025
// SolvedBy: taneshqGupta

use std::{
    collections::HashMap,
    // io::{stderr, Write},
};

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

fn is_red(n: usize, cin: &mut Scanner, setx: &mut HashMap<usize, bool>) -> bool {
    if !(setx.contains_key(&n)) {
        println!("? {}", n);
        let s: char = cin.next();
        setx.insert(n, s == 'R');
    }
    return *setx.get(&n).unwrap();
}

fn works(i1: usize, i1_is_red: bool, i2: usize, i2_is_red: bool) -> bool {
    if ((i1.abs_diff(i2) + 1) % 2 == 0) && (i1_is_red == i2_is_red) {
        return true;
    }
    if ((i1.abs_diff(i2) + 1) % 2 != 0) && (i1_is_red != i2_is_red) {
        return true;
    }
    false
}

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    let mut setx = HashMap::new();
    let mut low = 1;
    let mut high = n;
    while low < high {
        if low == (high - 1) {
            break;
        }
        let mid = low + (high - low) / 2;
        let low_is_red = is_red(low, &mut cin, &mut setx);
        let mid_is_red = is_red(mid, &mut cin, &mut setx);
        let high_is_red = is_red(high, &mut cin, &mut setx);
        if works(low, low_is_red, mid, mid_is_red) {
            high = mid;
            // dbg!(low, mid, high);
            // let _ = writeln!(stderr(), "\n");
            continue;
        } else if works(mid, mid_is_red, high, high_is_red) {
            low = mid;
            // dbg!(low, mid, high);
            // let _ = writeln!(stderr(), "\n");
            continue;
        } else {
            // dbg!(low, mid, high);
            // let _ = writeln!(stderr(), "\n");
            panic!("nooooooooo");
        }
    }
    if *setx.get(&(low + 1)).unwrap() == *setx.get(&low).unwrap() {
        println!("! {}", low);
    } else if *setx.get(&(low - 1)).unwrap() == *setx.get(&(low)).unwrap() {
        println!("! {}", low - 1);
    } else {
        panic!("noooooo");
    }
}
