use proconio::input;
use std::cmp::max;
use std::cmp::min;

fn main() {
    input! {
        n: i64,
        k: i64
    }
    println!("{}", min(n % k, k - n % k));
}
