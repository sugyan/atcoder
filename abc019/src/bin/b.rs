use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let answer = s
        .chars()
        .fold(Vec::new(), |mut v, c| {
            match v.last_mut() {
                Some((p, n)) if *p == c => *n += 1,
                _ => v.push((c, 1)),
            }
            v
        })
        .iter()
        .fold(String::new(), |mut s, &(c, n)| {
            s.push(c);
            s += &n.to_string();
            s
        });
    println!("{}", answer);
}
