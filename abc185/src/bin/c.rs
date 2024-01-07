use proconio::input;

fn main() {
    input! {
        l: i64
    }
    let mut res: i64 = 1;
    for i in 1..=11 {
        res *= l - i;
        res /= i;
    }
    println!("{}", res);
}
