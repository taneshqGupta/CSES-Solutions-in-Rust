// CSES Introductory-Problems Q-22 :: Digit Queries
// DateSolved: 6 April 2026
// SolvedBy: taneshqGupta

use std::array;

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
    // ---- TO CALCULATE K_VALUES ----
    // const MAX_K: usize = 1e20 as usize;
    // let mut level: usize = 1;
    // let mut k: usize = 9;
    // loop {
    //     if k <= MAX_K {
    //         println!("{}, ", k);
    //     } else {
    //         break;
    //     }
    //     k = k + (usize::pow(10, level as u32 + 1) - usize::pow(10, level as u32)) * (level + 1);
    //     level = level + 1;
    // }

    const K_VALUES: [usize; 18] = [
        9,
        189,
        2889,
        38889,
        488889,
        5888889,
        68888889,
        788888889,
        8888888889,
        98888888889,
        1088888888889,
        11888888888889,
        128888888888889,
        1388888888888889,
        14888888888888889,
        158888888888888889,
        1688888888888888889,
        17888888888888888889,
    ];
    const TEN_POWERS: [usize; 20] = [
        1,
        10,
        100,
        1000,
        10000,
        100000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
        1000000000000,
        10000000000000,
        100000000000000,
        1000000000000000,
        10000000000000000,
        100000000000000000,
        1000000000000000000,
        10000000000000000000,
    ];
    let mut cin = Scanner::default();
    let q: usize = cin.next();
    'outer: for _ in 0..q {
        let k: usize = cin.next();
        let mut no_of_digits: usize = 0;
        for i in 0..17 {
            if k <= K_VALUES[i] {
                no_of_digits = i + 1;
                break;
            }
        }
        if no_of_digits == 1 {
            println!("{}", k);
            continue 'outer;
        }
        let mut base_k = K_VALUES[no_of_digits - 2];
        // dbg!(base_k);
        let mut digits = Vec::new();
        let mut k_dividers: [usize; 10] = array::from_fn(|_| 0);
        let mut cont_k = base_k;
        for i in 0..10 {
            k_dividers[i] = cont_k;
            cont_k = cont_k + no_of_digits * TEN_POWERS[no_of_digits - 1];
        }
        // dbg!(no_of_digits, &k_dividers);
        for i in (0..=9).rev() {
            if k == k_dividers[i] {
                println!("9");
                continue 'outer;
            }
            if k > k_dividers[i] {
                base_k = k_dividers[i];
                digits.push(i + 1);
                break;
            }
        }
        // dbg!(base_k, &digits);
        for lvl in (1..no_of_digits).rev() {
            let mut k_dividers: [usize; 10] = array::from_fn(|_| 0);
            let mut cont_k = base_k;
            for i in 0..10 {
                k_dividers[i] = cont_k;
                cont_k = cont_k + no_of_digits * TEN_POWERS[lvl - 1];
            }
            // dbg!(no_of_digits, lvl, &k_dividers);
            for i in (0..=9).rev() {
                if k > k_dividers[i] {
                    base_k = k_dividers[i];
                    digits.push(i);
                    break;
                }
            }
            // dbg!(base_k, &digits);
        }
        // let mut base_number = 0;
        // for i in 1..=no_of_digits {
        //     base_number += digits[no_of_digits - i] * TEN_POWERS[i - 1];
        // }
        // dbg!(base_number);
        println!("{}", digits[k - base_k - 1]);
    }
}
