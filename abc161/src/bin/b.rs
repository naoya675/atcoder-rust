use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        a: [i32; n]
    }
    let sum: i32 = a.iter().sum();
    let mut cnt = 0;
    for ai in a {
        if ai as f64 >= sum as f64 / (4.0 * m as f64) {
            cnt += 1;
        }
    }
    if cnt >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
