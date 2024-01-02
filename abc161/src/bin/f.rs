use proconio::input;

fn main() {
    input! {
        n: i64
    }
    let mut pattern = divisors(n - 1).len() - 1;
    for d in divisors(n) {
        if d == 1 {
            continue;
        }
        let mut n = n;
        while n % d == 0 {
            n /= d;
        }
        if n % d == 1 {
            pattern += 1;
        }
    }
    println!("{}", pattern);
}

fn divisors(n: i64) -> Vec<i64> {
    let mut divisor = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            if n / i == i {
                divisor.push(i);
            } else {
                divisor.push(i);
                divisor.push(n / i);
            }
        }
        i += 1;
    }
    divisor
}
