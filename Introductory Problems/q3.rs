// CSES ProblemSet Q-3 :: Repetitions
// DateSolved: 24 Jun 2025
// SolvedBy: taneshqGupta

use crate::helpers::*;
use std::io::{Read, Write};

fn solve<'a>(cin: &mut impl Iterator<Item = &'a str>, cout: &mut impl Write) -> Option<()> {

    const N: usize = 2e6 as usize;
    let mut aa: [u32; N] = [0; N];
    let s: String = get(cin)?;
    let n: usize = s.len();
    let ss: Vec<char> = s.chars().collect();

    for i in 1..n {
        if ss[i] == ss[i-1] {
            aa[i] = aa[i-1] + 1;
        } 
    } 

    let ans: u32 = *aa.iter().max().unwrap();
    set(cout, &ans + 1); nl(cout);

    Some(())
}

#[allow(dead_code)]
mod helpers {
    use std::{fmt::Display, io::Write, str::FromStr};

    pub fn set<T: Display>(cout: &mut impl Write, a: T) {
        write!(cout, "{}", a).ok();
    }

    pub fn get<'a, T: FromStr>(cin: &mut impl Iterator<Item = &'a str>) -> Option<T> {
        Some(cin.next()?.parse::<T>().ok()?)
    }

    pub fn get_vec<'a, T: FromStr>(
        cin: &mut impl Iterator<Item = &'a str>,
        n: usize,
    ) -> Option<Vec<T>> {
        Some(cin.take(n).filter_map(|s| s.parse::<T>().ok()).collect())
    }

    pub fn set_vec<T: Display>(cout: &mut impl Write, aa: &[T]) {
        let mut iter = aa.iter().peekable();
        while let Some(item) = iter.next() {
            write!(cout, "{}", item).ok();
            if iter.peek().is_some() {
                write!(cout, " ").ok();
            }
        }
    }

    pub fn nl(cout: &mut impl Write) {
        writeln!(cout, "").ok();
    }

    pub fn sp(cout: &mut impl Write) {
        write!(cout, " ").ok();
    }
}

fn main() {
    let mut cout = std::io::BufWriter::new(std::io::stdout());
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut cin = s.split_ascii_whitespace();

    // -- if there are test cases --
    // let _t: usize = get(&mut cin).unwrap();
    // -- if there are test cases --

    while let Some(_) = solve(&mut cin, &mut cout) {}
}
