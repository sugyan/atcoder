use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c: usize,
        mut nml: [[u32; 3]; c],
    }
    let mut max = [0; 3];
    for size in nml.iter_mut() {
        size.sort_unstable();
        (0..3).for_each(|i| max[i] = max[i].max(size[i]));
    }
    println!("{}", max.iter().product::<u32>());
}
