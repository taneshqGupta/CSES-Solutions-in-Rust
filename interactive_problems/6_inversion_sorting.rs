// CSES Interactive-Problems Q-6 ::  Inversion Sorting
// DateSolved: 7 Sep 2025
// SolvedBy: taneshqGupta

use std::collections::BTreeSet;

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

fn reversex(a: &mut Vec<usize>, mut i: usize, mut j: usize) {
    if i == j {
        return;
    }
    while i < j {
        let temp = a[i];
        a[i] = a[j];
        a[j] = temp;
        i += 1;
        j -= 1;
    }
}

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    if n == 1 {
        println!("1 1");
        let _ = cin.next::<usize>();
        return;
    }
    if n == 2 {
        println!("1 2");
        let m = cin.next::<usize>();
        if m == 0 {
            return;
        } else {
            println!("1 2");
            let _ = cin.next::<usize>();
            return;
        }
    }
    let mut inv = Vec::new();
    inv.push(0);
    for i in 2..=n {
        println!("{} {}", 1, i);
        let finalo = cin.next::<usize>();
        println!("{} {}", 1, i);
        let initialo = cin.next::<usize>();
        let i_c2 = i * (i - 1) / 2;
        inv.push((i_c2 + initialo - finalo) / 2);
    }
    // dbg!(&inv);
    let mut b = vec![0; n];
    b[0] = 0;
    for i in 1..n {
        b[i] = inv[i] - inv[i - 1];
    }
    // dbg!(&b);
    let mut a = vec![0; n];
    let mut setx: BTreeSet<usize> = (1..=n).collect();
    for i in (0..n).rev() {
        let k = *setx.iter().nth(setx.len() - b[i] - 1).unwrap();
        a[i] = k;
        setx.remove(&k);
    }
    // dbg!(&a);
    for i in 1..=n {
        let mut hoho = 0;
        for x in 0..a.len() {
            if a[x] == i {
                hoho = x;
            }
        }
        println!("{} {}", i, hoho + 1);
        reversex(&mut a, i - 1, hoho);
        // dbg!(&a);
        let m = cin.next::<usize>();
        if m == 0 {
            std::process::exit(0);
        }
    }
}
