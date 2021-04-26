use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        mut a: [i64; n]
    };

    a.sort();

    let mut ans: i64 = 1;

    for i in 0..n {
        match ans.checked_mul(a[i]){
            Some(mul) => { ans = mul },
            None => { ans = -1; break;}
        }
        if ans > 1_000_000_000_000_000_000 {
            ans = -1; break;
        }
    }

    println!("{}", ans);
}
