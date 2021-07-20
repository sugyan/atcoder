use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xyh: [(i32, i32, i32); n],
    }
    let i = xyh.iter().position(|xyh| xyh.2 > 0).unwrap();
    for x in 0..=100 {
        for y in 0..=100 {
            let h = (xyh[i].0 - x).abs() + (xyh[i].1 - y).abs() + xyh[i].2;
            if xyh
                .iter()
                .all(|xyh| (h - (xyh.0 - x).abs() - (xyh.1 - y).abs()).max(0) == xyh.2)
            {
                println!("{} {} {}", x, y, h);
                return;
            }
        }
    }
}
