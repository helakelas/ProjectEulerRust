use core::util::unreachable;

use common::monoid::{ Sum, merge_as, mergei_as };

pub struct Prime {
    priv vec: ~[uint],
}

impl Prime {
    #[inline(always)]
    pub fn new() -> Prime { Prime { vec: ~[] } }

    #[inline(always)]
    priv fn is_coprime(&mut self, num: uint) -> bool {
        for self.vec.each |&p| {
            if p * p > num  { return true; }
            if num % p == 0 { return false; }
        }
        return true;
    }

    #[inline(always)]
    priv fn grow_len(&mut self, len: uint) {
        if self.vec.len() >= len { return; }

        let mut num;
        if self.vec.is_empty() {
            self.vec.push(2);
            if self.vec.len() >= len { return; }
            num = 2;
        } else {
            num = self.vec.last().clone();
        }

        if num == 2 {
            self.vec.push(3);
            if self.vec.len() >= len { return; }
            num = 3;
        }

        while self.vec.len() < len {
            num += 2;
            if self.is_coprime(num) {
                self.vec.push(num);
            }
        }
    }

    #[inline(always)]
    pub fn get_at(&mut self, idx: uint) -> uint {
        self.grow_len(idx + 1);
        return self.vec[idx];
    }

    #[inline(always)]
    pub fn is_prime(&mut self, num: uint) -> bool {
        if num < 2 { return false; }

        for self.each |p| {
            if p * p > num  { return true;  }
            if num % p == 0 { return false; }
        }
        unreachable();
    }

    #[inline(always)]
    pub fn each(&mut self, f: &fn(uint) -> bool) {
        for self.each_borrow |p, _ps| {
            if !f(p) {
                return;
            }
        }
        unreachable();
    }

    #[inline(always)]
    pub fn each_borrow(&mut self, f: &fn(uint, &mut Prime) -> bool) {
        let init_len = self.vec.len();
        for uint::range(0, init_len) |i| {
            let p = self.vec[i];
            if !f(p, self) { return; }
        }

        let mut idx = init_len;
        loop {
            let p = self.get_at(idx);
            if !f(p, self) { return; }
            idx += 1;
        }
    }
}

priv struct Factors<'self> {
    priv num: uint,
    priv prime: &'self mut Prime
}

impl<'self> BaseIter<(uint, int)> for Factors<'self> {
    #[inline(always)]
    fn each(&self, blk: &fn(v: &(uint, int)) -> bool) {
        if self.num == 0 { return; }

        let mut itr = self.num;
        for self.prime.each |p| {
            let mut exp = 0;
            while itr % p == 0 {
                exp += 1;
                itr /= p;
            }
            // let exp = div_multi(&mut itr, p);
            if exp > 0 {
                if !blk(&(p, exp as int)) { break; }
            }
            if itr == 1 { break; }
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> Option<uint> { None }
}

impl<'self> Factors<'self> {
    #[inline(always)]
    pub fn new<'a>(num: uint, primes: &'a mut Prime) -> Factors<'a> {
        Factors { num: num, prime: primes }
    }
}

#[inline(always)]
priv fn pow(base: uint, exp: uint) -> uint {
    let mut result = 1;
    let mut itr = exp;
    let mut pow = base;
    while itr > 0 {
        if itr & 0x1 == 0x1 {
            result *= pow;
        }
        itr >>= 1;
        pow *= pow;
    }
    return result;
}

#[inline(always)]
pub fn factors_to_uint<IA: BaseIter<(uint, int)>>(fs: &IA) -> uint {
    let mut result = 1;
    for fs.each() |&tp| {
        let (base, exp) = tp;
        if exp > 0 {
            result *= pow(base, exp as uint);
        } else {
            result /= pow(base, (-exp) as uint);
        }
    }
    return result;
}

#[inline(always)]
pub fn num_of_divisors(num: uint, primes: &mut Prime) -> uint {
    if num == 0 { return 0; }
    let mut prod = 1;
    for Factors::new(num, primes).each |&f| {
        let (_base, exp) = f;
        prod *= (exp as uint) + 1;
    }
    return prod;
}

#[inline(always)]
pub fn sum_of_divisors(num: uint, primes: &mut Prime) -> uint {
    if num == 0 { return 0; }
    let mut sum = 1;
    for Factors::new(num, primes).each |&f| {
        let (base, exp) = f;
        sum *= (pow(base, (exp as uint) + 1) - 1) / (base - 1);
    }
    return sum;
}

pub fn sum_of_proper_divisors(num: uint, primes: &mut Prime) -> uint {
    sum_of_divisors(num, primes) - num
}

pub fn comb(n: uint, r: uint, ps: &mut Prime) -> uint {
    let mut numer_facts = ~[];
    for uint::range(r + 1, n + 1) |i| {
        numer_facts.push(iter::to_vec(&Factors::new(i, ps)));
    }
    let numer = mergei_as(numer_facts, Sum);

    let mut denom_facts = ~[];
    for uint::range(1, n - r + 1) |num| {
        denom_facts.push(iter::to_vec(&Factors::new(num, ps)));
    }
    let denom = mergei_as(denom_facts, |i| Sum(-i));

    return factors_to_uint(&merge_as(numer, denom, Sum));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_opidx () {
        let table  = [  2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];
        let mut ps = Prime::new();

        // Generated primes
        for table.eachi() |i, p| { assert_eq!(ps.get_at(i), *p); }
        // Memoized primes
        for table.eachi() |i, p| { assert_eq!(ps.get_at(i), *p); }
    }

    #[test]
    fn test_prime_each() {
        let table  = ~[  2,  3,  5,  7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ];
        let table2 = ~[ 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97 ];
        let mut ps = Prime::new();

        let mut v1 = ~[];
        for ps.each |p| {
            if p > *table.last() { break; }
            v1 += [ p ];
        }
        assert_eq!(table.initn(0), v1.initn(0));

        let mut v2 = ~[];
        for ps.each |p| {
            if p > *table.last() { break; }
            v2 += [ p ];
        }
        assert_eq!(table.initn(0), v2.initn(0));

        let mut v3 = ~[];
        for ps.each |p| {
            if p > *table2.last() { break; }
            v3 += [ p ];
        }
        assert_eq!(table + table2, v3);
    }

    #[test]
    fn test_prime_is_prime() {
        let mut p = Prime::new();
        assert!(!p.is_prime(0));
        assert!(!p.is_prime(1));
        assert!(p.is_prime(2));
        assert!(p.is_prime(3));
        assert!(!p.is_prime(4));
        assert!(p.is_prime(5));
        assert!(!p.is_prime(6));
        assert!(p.is_prime(7));
        assert!(!p.is_prime(100));
    }

    #[test]
    fn test_factors() {
        let mut p = Prime::new();
        for Factors::new(1, &mut p).each |_f| {
            fail!();
        }

        for Factors::new(8, &mut p).each |&f| {
            assert_eq!(f, (2, 3));
        }

        let mut v = ~[(2, 3), (3, 3)];
        for Factors::new(8 * 27, &mut p).each |&f| {
            assert_eq!(f, v.shift());
        }
    }
}