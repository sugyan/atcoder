use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
        t: String,
    }
    let mut answer = String::new();
    for u in t.bytes() {
        answer += &s[(u - b'1') as usize];
    }
    println!("{}", answer);
}
