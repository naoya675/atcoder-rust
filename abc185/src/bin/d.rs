use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        mut a: [i32; m as usize]
    }
    if n == m {
        println!("{}", 0);
        return;
    }
    a.push(0);
    a.push(n + 1);
    a.sort();
    let mut k = i32::MAX;
    for a in a.windows(2) {
        if a[1] - a[0] - 1 > 0 {
            k = k.min(a[1] - a[0] - 1);
        }
    }
    let mut res = 0;
    for a in a.windows(2) {
        if a[1] - a[0] - 1 > 0 {
            res += (a[1] - a[0] - 1 + k - 1) / k;
        }
    }
    println!("{}", res);
}
