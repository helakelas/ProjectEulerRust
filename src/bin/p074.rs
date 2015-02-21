//! [Problem 74](https://projecteuler.net/problem=74) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#[macro_use(problem)] extern crate common;

use std::collections::HashMap;
use std::iter;

#[derive(Clone)]
enum Length { Loop(usize), Chain(usize), Unknown }

fn fact_sum(mut n: u32, fs: &[u32; 10]) -> u32 {
    if n == 0 { return 1; }

    let mut sum = 0;
    while n > 0 {
        sum += fs[(n % 10) as usize];
        n /= 10;
    }
    sum
}

fn get_chain_len(n: u32, map: &mut[Length], fs: &[u32; 10]) -> usize {
    let mut chain_map = HashMap::new();
    let mut idx = n;
    let mut chain_len = 0;
    let mut loop_len  = 0;

    loop {
        match map[idx as usize] {
            Length::Loop(c)  => { loop_len += c;  break; }
            Length::Chain(c) => { chain_len += c; break; }
            Length::Unknown  => {
                match chain_map.get(&idx) {
                    Some(&chain_idx) => {
                        loop_len  = chain_len - chain_idx;
                        chain_len = chain_idx;
                        break;
                    }
                    None => {
                        let _ = chain_map.insert(idx, chain_len);
                        idx = fact_sum(idx, fs);
                        chain_len += 1;
                    }
                }
            }
        }
    }

    for (&key, &idx) in chain_map.iter() {
        if idx >= chain_len {
            map[key as usize] = Length::Loop(loop_len);
        } else {
            map[key as usize] = Length::Chain(loop_len + chain_len - idx);
        }
    }

    chain_len + loop_len
}

fn solve() -> String {
    let limit = 1000000;
    let factorial = {
        let mut val = [1; 10];
        for i in (1usize .. 10) {
            val[i] = val[i - 1] * (i as u32);
        }
        val
    };

    let mut map = iter::repeat(Length::Unknown)
        .take((factorial[9] * 6 + 1) as usize)
.collect::<Vec<_>>();
    let mut cnt = 0;
    for n in (1u32 .. limit + 1) {
        let len = get_chain_len(n, map.as_mut_slice(), &factorial);
        if len == 60 { cnt += 1; }
    }

    cnt.to_string()
}

problem!("402", solve);

#[cfg(test)]
mod tests {
    use std::iter;

    #[test]
    fn len() {
        let factorial = {
            let mut val = [1; 10];
            for i in (1usize .. 10) {
                val[i] = val[i - 1] * (i as u32);
            }
            val
        };
        let mut map = iter::repeat(super::Length::Unknown).take((factorial[9] * 6 + 1) as usize).collect::<Vec<_>>();

        assert_eq!(3, super::get_chain_len(169, map.as_mut_slice(), &factorial));
        assert_eq!(2, super::get_chain_len(871, map.as_mut_slice(), &factorial));
        assert_eq!(2, super::get_chain_len(872, map.as_mut_slice(), &factorial));
        assert_eq!(5, super::get_chain_len(69, map.as_mut_slice(), &factorial));
        assert_eq!(4, super::get_chain_len(78, map.as_mut_slice(), &factorial));
        assert_eq!(2, super::get_chain_len(540, map.as_mut_slice(), &factorial));
    }

}
