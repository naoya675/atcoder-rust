use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        t: i32,
        ab: [(i32, i32); m as usize]
    }
    let mut battery = n;
    let mut current = 0;
    for (a, b) in ab {
        battery -= a - current;
        if battery <= 0 {
            break;
        }
        battery += b - a;
        battery = battery.min(n);
        current = b;
    }
    battery -= t - current;
    println!("{}", if battery > 0 { "Yes" } else { "No" });
}
