use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::collections::HashMap;

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        mut n: i64
    };
    let mut ans = 0;

    let factor = prime_factor(n);

    for (p, num) in factor {
        for e in 1..=num {
            let z = p.pow(e as u32);
            if n % z == 0{
                n = n / z;
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}


pub fn prime_factor(mut n: i64) -> HashMap<i64, i64> {
    let mut res: HashMap<i64, i64> = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            let v = match res.get(&i) {
                Some(v) => *v + 1,
                None => 1,
            };
            res.insert(i, v);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.insert(n, 1);
    }
    res
}