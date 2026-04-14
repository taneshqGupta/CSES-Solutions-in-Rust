// CSES Sorting & Searching Q-13 :: Playlist
// DateSolved: 14 April 2026
// SolvedBy: taneshqGupta

use std::collections::HashSet;

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
    let playlist: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut maxlength = 1;
    let mut l = 0;
    let mut r = 1;
    let mut setx: HashSet<usize> = HashSet::new();
    setx.insert(playlist[0]);
    
    while l < n && r < n {
        // dbg!(l, r);
        while setx.contains(&playlist[r]) {
            setx.remove(&playlist[l]);
            l += 1;
        }
        setx.insert(playlist[r]);
        maxlength = maxlength.max(setx.len());
        // dbg!(&setx, maxlength, l, r);
        r += 1;
    }

    println!("{}", maxlength);
}