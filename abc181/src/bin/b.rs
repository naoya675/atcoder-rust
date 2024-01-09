use proconio::input;

fn main() {
    input! {
        n: i32,
        ab: [(i64, i64); n as usize]
    }
    let mut res = 0;
    for (a, b) in ab {
        res += (a + b) * (b - a + 1) / 2;
    }
    println!("{}", res);
}
