use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [u32; 3],
        k: usize,
    }
    abc.sort();
    println!("{}", abc[0] + abc[1] + (abc[2] << k));
}
