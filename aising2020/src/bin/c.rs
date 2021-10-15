use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
    }
    for i in 1..=n {
        let mut answer = 0;
        for x in (1..).take_while(|&x| x * x < i) {
            let j = i - x * x;
            for y in (x..).take_while(|&y| y * y + x * y < j) {
                let k = j - y * y - x * y;
                let (mut lo, mut hi) = (y, k);
                while lo < hi {
                    let m = (lo + hi) / 2;
                    if m * (x + y + m) < k {
                        lo = m + 1;
                    } else {
                        hi = m;
                    }
                }
                if lo * (x + y + lo) == k {
                    answer += if x == y && y == lo {
                        1
                    } else if x == y || y == lo || x == lo {
                        3
                    } else {
                        6
                    };
                }
            }
        }
        println!("{}", answer);
    }
}
