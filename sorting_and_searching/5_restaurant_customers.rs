// CSES Sorting & Searching Q-5 :: Restaurant Customers
// DateSolved: 10 April 2026
// SolvedBy: taneshqGupta

use std::collections::{BTreeMap, HashMap};

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
    let mut times: Vec<(usize, usize)> = (0..n).map(|_| (cin.next(), cin.next())).collect();
    times.sort_unstable();
    let mapx: BTreeMap<usize, usize> = (0..n).map(|i| (times[i].0, times[i].1)).collect();
    let mut how_many_byebyes_after_this: HashMap<usize, isize> = HashMap::new();

    for i in 0..n {
        let go_time = times[i].1;
        let (&just_before_byebye, _) = mapx.range(..go_time).next_back().unwrap();
        *how_many_byebyes_after_this
            .entry(just_before_byebye)
            .or_insert(0) += 1;
    }

    // dbg!(&how_many_byebyes_after_this);

    let mut timer = 0;
    let mut max_time = 0;

    for i in 0..n {
        let entrytime = times[i].0;
        // dbg!(&entrytime);
        timer += 1;
        if timer > max_time {
            max_time = timer;
        }
        if how_many_byebyes_after_this.contains_key(&entrytime) {
            // dbg!("contains byebyes");
            let byebyes = *how_many_byebyes_after_this.get(&entrytime).unwrap();
            timer -= byebyes;
            // dbg!(&byebyes);
        }
        // dbg!(&timer, &max_time);
    }

    println!("{}", max_time);
}
