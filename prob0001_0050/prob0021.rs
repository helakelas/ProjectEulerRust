use common::prime::{ Prime, sum_of_proper_divisors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 21,
    answer: "31626",
    solver: solve
};

fn solve() -> ~str {
    let mut p = Prime::new();
    let elms = vec::from_fn(10000, |n| sum_of_proper_divisors(n, &mut p));

    let mut amicables = ~[];
    for elms.eachi |n, sum| {
        if *sum >= n { loop }
        if *sum < elms.len() && elms[*sum] == n {
            amicables += [(*sum, n)];
        }
    }

    let mut sum = 0;
    for amicables.each |pair| {
        let (a, b) = *pair;
        sum += a + b;
    }
    return sum.to_str();
}