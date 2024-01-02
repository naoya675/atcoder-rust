use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: i32,
        k: i32,
        c: i32,
        s: Chars
    }
    let n: usize = n as usize;
    let k: usize = k as usize;
    let mut pre = Vec::new();
    let mut suf = Vec::new();
    let mut cnt = c;
    for i in 0..n {
        if s[i] == 'o' && cnt >= c {
            pre.push(i);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }
    let mut cnt = c;
    for i in (0..n).rev() {
        if s[i] == 'o' && cnt >= c {
            suf.push(i);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }
    suf.reverse();
    if pre.len() == k {
        for i in 0..k {
            if pre[i] == suf[i] {
                println!("{}", pre[i] + 1);
            }
        }
    }
}
