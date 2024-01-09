use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut count = vec![0; 10];
    for &c in &s {
        count[c as usize - '0' as usize] += 1;
    }
    if s.len() == 1 {
        for i in (8..10).step_by(8) {
            let mut count = count.to_vec();
            for c in i.to_string().chars() {
                count[c as usize - '0' as usize] -= 1;
            }
            if count.iter().all(|x| x >= &0) {
                println!("Yes");
                return;
            }
        }
    }
    if s.len() == 2 {
        for i in (16..100).step_by(8) {
            let mut count = count.to_vec();
            for c in i.to_string().chars() {
                count[c as usize - '0' as usize] -= 1;
            }
            if count.iter().all(|x| x >= &0) {
                println!("Yes");
                return;
            }
        }
    }
    if s.len() >= 3 {
        for i in (104..1000).step_by(8) {
            let mut count = count.to_vec();
            for c in i.to_string().chars() {
                count[c as usize - '0' as usize] -= 1;
            }
            if count.iter().all(|x| x >= &0) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
