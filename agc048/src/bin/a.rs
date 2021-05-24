use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        s: [String; t],
    }
    for s in &s {
        let answer = if s.as_str() > "atcoder" {
            0
        } else if let Some(i) = s.chars().position(|c| c != 'a') {
            i as i32 - if s.as_bytes()[i] > b't' { 1 } else { 0 }
        } else {
            -1
        };
        println!("{}", answer);
    }
}
