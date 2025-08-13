// CSES Interactive-Problems Q-4 :: Permuted Binary Strings
// DateSolved: 14 Aug 2025
// SolvedBy: taneshqGupta 

// use std::io::{stderr, Write};

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

    #[derive(Debug)]
    struct Node {
        low_bound: usize,
        high_bound: usize,
    }

    let mut a: Vec<Node> = (0..n)
        .map(|_| Node {
            low_bound: 1,
            high_bound: n,
        })
        .collect();

    let mut divs = [false; 1001];
    divs[n] = true;
    loop {
        let mut ys = true;
        for i in 1..=n {
            if !divs[i] {
                ys = false;
            }
        }
        if ys {
            break;
        }

        let mut last = 0;
        for i in 1..=n {
            if divs[i] {
                divs[last + (i - last) / 2] = true;
                last = i;
            }
        }

        let mut queries = Vec::new();
        let mut zero = true;
        for i in 1..=n {
            queries.push(!zero);
            if divs[i] {
                zero = !zero;
            }
        }
        print!("? ");
        for &i in &queries {
            print!("{}", if i { 1 } else { 0 });
        }
        println!();
        let s: String = cin.next();

        // for i in 1..=n {
        //     let _ = write!(stderr(), "{} ", if divs[i] { 1 } else { 0 });
        // }
        // let _ = writeln!(stderr());

        let mut boolx = true;
        for i in 1..=n {
            if !divs[i] {
                boolx = false;
            }
        }
        for (i, c) in s.chars().enumerate() {
            if a[i].low_bound == a[i].high_bound {
                continue;
            }
            if !boolx {
                if c == '0' {
                    let mut z = a[i].low_bound;
                    loop {
                        if divs[z] {
                            a[i].high_bound = z;
                            break;
                        }
                        z += 1;
                    }
                } else {
                    let mut z = a[i].high_bound - 1;
                    loop {
                        if divs[z] {
                            a[i].low_bound = z + 1;
                            break;
                        }
                        z -= 1;
                    }
                }
            } else {
                if c == '0' {
                    match (a[i].low_bound % 2 == 1, a[i].high_bound % 2 == 1) {
                        (true, true) => {
                            a[i].high_bound = a[i].low_bound;
                        }
                        (true, false) => {
                            a[i].high_bound = a[i].low_bound;
                        }
                        (false, true) => {
                            a[i].low_bound = a[i].high_bound;
                        }
                        (false, false) => {
                            a[i].high_bound = a[i].low_bound;
                        }
                    }
                } else {
                    match (a[i].low_bound % 2 == 0, a[i].high_bound % 2 == 0) {
                        (true, true) => {
                            a[i].high_bound = a[i].low_bound;
                        }
                        (true, false) => {
                            a[i].high_bound = a[i].low_bound;
                        }
                        (false, true) => {
                            a[i].low_bound = a[i].high_bound;
                        }
                        (false, false) => {
                            a[i].high_bound = a[i].low_bound;
                        }
                    }
                }
            }
        }
        drop(s);
        // dbg!(&a);
    }
    print!("! ");
    for node in a {
        print!("{} ", node.low_bound);
    }
    println!();
}