use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut v = Vec::new();
    for &a in a.iter().rev() {
        match v.binary_search(&(a + 1)) {
            Ok(i) => v[i] = a,
            Err(i) if i < v.len() => v[i] = a,
            _ => v.push(a),
        }
    }
    println!("{}", v.len());
}
