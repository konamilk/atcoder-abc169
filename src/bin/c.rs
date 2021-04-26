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
        a: i64,
        b_str: String
    };

    let spliced = b_str.split(".").collect::<Vec<&str>>();

    let b_100: i64;
    if spliced.len() == 1 {
        b_100 = (spliced[0].parse::<i64>().unwrap()) * 100
    }
    else {
        b_100 = spliced[0].parse::<i64>().unwrap() * 100 + spliced[1].parse::<i64>().unwrap()
    }

    let ans = a * b_100 / 100;

    println!("{}", ans);

}
