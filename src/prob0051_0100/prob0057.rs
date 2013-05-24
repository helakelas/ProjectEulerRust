#[link(name = "prob0057", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use extra::bigint::{ BigUint };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 57,
    answer: "153",
    solver: solve
};

// a[0] = 1 + 1/2
// a[1] = 1 + 1/(2 + 1/2)
//      = 1 + 1/(1 + a[0]) = 
// a[2] = 1 + 1/(2 + 1/(2 + 1/2))
//      = 1 + 1/(1 + a[1])
// a[i+1] = n[i+1] / d[i+1]
//        = 1 + 1 / (1 + n[i]/d[i])
//        = 1 + d[i] / (d[i] + n[i])
//        = (2d[i] + n[i]) / (d[i] + n[i])
//  n[0] = 3, d[0] = 2
//  n[i+1] = 2d[i] + n[i]
//  d[i+1] = d[i] + n[i]

fn each_frac(f: &fn(&BigUint, &BigUint) -> bool) -> bool {
    let mut n = BigUint::from_uint(3);
    let mut d = BigUint::from_uint(2);
    loop {
        if !f(&n, &d) { return false; }
        let new_n = BigUint::from_uint(2) * d + n;
        let new_d = n + d;
        n = new_n;
        d = new_d;
    }
}

pub fn solve() -> ~str {
    let mut i = 0;
    let mut cnt = 0u;
    for each_frac |n, d| {
        i += 1;
        let n_len = n.to_str().len();
        let d_len = d.to_str().len();
        if n_len > d_len { cnt += 1; }
        if i >= 1000 { break; }
    }
    return cnt.to_str();
}
