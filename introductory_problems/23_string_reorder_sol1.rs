// CSES Introductory-Problems Q-23 :: String Reorder
// DateSolved: 8 April 2026
// SolvedBy: taneshqGupta

use std::{cmp::Reverse, collections::BTreeMap};

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
    let s: String = cin.next();
    let n = s.len();
    let mut f: [usize; 26] = [0; 26];
    for c in s.bytes() {
        f[(c - b'A') as usize] += 1;
    }
    // dbg!(&f);

    /* For every position from 0..n,
    I want to consider all the alphabets in order,
    then I want to reduce the frequency of
    the alphabet in hand by one
    and then see if the remaining string can be constructed.

    For this we will calculate if the remaining string's
    max frequency element appears more than ceil(N/2) times.

    Main crux is what data structure do we use to
    both be able to get the max element in O(1) but also
    be able to update current element in O(1).
    What if we make a frequency map of frequencies themself?
    In that case, we can use "ffmap.pop_first()" to
    get the highest frequency. We can also update
    middle elements in O(log(max possible number of frequencies)),
    which is O(log(26)) = O(5).
    This should result in T.C = O(n * 26 * 5) = O(130 * n)
    This is borderline acceptable for n = 1e6. */

    let mut result: Vec<u8> = Vec::with_capacity(n);

    let mut ffmap: BTreeMap<Reverse<usize>, u8> = BTreeMap::new();
    for i in 0..26 {
        *ffmap.entry(Reverse(f[i])).or_insert(0) += 1;
    }
    // ffmap.pop_first();
    // dbg!(&ffmap);

    let mut last_char = None;
    for i in 0..n {
        let remaining_length = n - 1 - i;
        let mut found = false;
        for c in 0..26 {
            if f[c] > 0 && Some(c) != last_char {
                // Decrement the current frequency by 1
                // This means we would need to decrement
                // the current frequency f[c] in ffmap and
                // increment 'f[c] - 1' in ffmap
                if ffmap.get(&Reverse(f[c])) == Some(&1) {
                    ffmap.remove_entry(&Reverse(f[c]));
                } else {
                    *ffmap.get_mut(&Reverse(f[c])).unwrap() -= 1;
                }
                f[c] -= 1;
                *ffmap.entry(Reverse(f[c])).or_insert(0) += 1;

                let f_max = *ffmap.first_entry().unwrap().key();
                let f_max = f_max.0;
                if f_max <= remaining_length.div_ceil(2) {
                    found = true;
                    last_char = Some(c);
                    result.push(c as u8);
                    break;
                }

                // Reverse the effects
                if ffmap.get(&Reverse(f[c])) == Some(&1) {
                    ffmap.remove_entry(&Reverse(f[c]));
                } else {
                    *ffmap.get_mut(&Reverse(f[c])).unwrap() -= 1;
                }
                f[c] += 1;
                *ffmap.entry(Reverse(f[c])).or_insert(0) += 1;
            }
        }
        if !found {
            println!("-1");
            return;
        }
    }
    for c in result {
        print!("{}", (c + b'A') as char);
    }
}
