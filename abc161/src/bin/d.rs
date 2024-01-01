use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        k: i32
    }
    let mut que: VecDeque<i64> = VecDeque::new();
    for i in 1..10 {
        que.push_back(i);
    }
    for _ in 1..k {
        if let Some(q) = que.pop_front() {
            if q % 10 != 0 {
                que.push_back(q * 10 + (q % 10) - 1);
            }
            que.push_back(q * 10 + (q % 10));
            if q % 10 != 9 {
                que.push_back(q * 10 + (q % 10) + 1);
            }
        }
    }
    if let Some(q) = que.pop_front() {
        println!("{}", q);
    }
}
