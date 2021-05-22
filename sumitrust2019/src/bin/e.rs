use proconio::{fastout, input};

const DIV: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut answer = 1;
    let mut rgb = [0, 0, 0];
    for &a in &a {
        let c = (0..3).filter(|&i| rgb[i] == a).collect::<Vec<_>>();
        answer = (answer * c.len() as u64) % DIV;
        if !c.is_empty() {
            rgb[c[0]] += 1;
        }
    }
    println!("{:?}", answer);
}
