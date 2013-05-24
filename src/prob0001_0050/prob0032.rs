#[link(name = "prob0032", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::hashmap::{ HashSet };
use extra::sort::{ merge_sort };
use common::calc::{ num_to_digits, permutate_num };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 32,
    answer: "45228",
    solver: solve
};

// possible num of digits combinations
// 1 x 1 = 7 : NG 10 * 10
// 1 x 2 = 6 : NG 10 * 100
// 1 x 3 = 5 : NG 10 * 1000 = 10000
// 1 x 4 = 4 : OK
// 2 x 2 = 5 : NG 100 * 100 = 10000
// 2 x 3 = 4 : OK
// 3 x 3 = 3 : NG

pub fn solve() -> ~str {
    let digits = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut answer = HashSet::new();

    // 1 x 4 = 4
    // a b = c
    // 1 < a < 10
    // 1000 < b < 10000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    //  => 1000 < b < 10000 / a
    for permutate_num(digits, 1, 0, 9) |a, ds| {
        for permutate_num(ds, 4, 1000, 9999 / a) |b, ds| {
            let c = a * b;
            let c_digits = merge_sort(num_to_digits(c, 10), |a, b| a <= b);
            if ds == c_digits { answer.insert(c); }
        }
    }

    // 2 x 3 = 4
    // a b = c
    // 10   < a < 100
    // 100  < b < 1000
    // 1000 < c = ab < 10000 => 1000 / a < b < 10000 / a
    // => 100 < b < 10000 / a
    for permutate_num(digits, 2, 10, 99) |a, ds| {
        for permutate_num(ds, 3, 100, 9999 / a) |b, ds| {
            let c = a * b;
            let c_digits = merge_sort(num_to_digits(c, 10), |a, b| a <= b);
            if ds == c_digits { answer.insert(c); }
        }
    }

    let mut sum = 0;
    for answer.each |&c| { sum += c; }
    return sum.to_str();
}