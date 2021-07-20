use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, t: u32,
        ct: [(u32, u32); n],
    }
    if let Some(min) = ct
        .iter()
        .filter_map(|ct| if ct.1 <= t { Some(ct.0) } else { None })
        .min()
    {
        println!("{}", min);
    } else {
        println!("TLE");
    }
}
