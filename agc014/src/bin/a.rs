use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [u32; 3],
    }
    if abc[0] & 1 == 0 && abc[0] == abc[1] && abc[1] == abc[2] {
        println!("-1");
        return;
    }
    let mut answer = 0;
    while abc.iter().all(|m| m & 1 == 0) {
        abc = vec![
            (abc[1] + abc[2]) / 2,
            (abc[0] + abc[2]) / 2,
            (abc[0] + abc[1]) / 2,
        ];
        answer += 1;
    }
    println!("{}", answer);
}
