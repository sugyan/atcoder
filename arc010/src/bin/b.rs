use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        md: [String; n],
    }
    let holidays = md
        .iter()
        .map(|s| {
            s.split('/')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut days = (0..366)
        .map(|i| i % 7 == 0 || i % 7 == 6)
        .collect::<Vec<_>>();
    let offset = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30];
    for holiday in &holidays {
        let mut day = holiday[1] - 1;
        for &o in offset.iter().take(holiday[0]) {
            day += o;
        }
        while day < 366 && days[day] {
            day += 1;
        }
        if day < 366 {
            days[day] = true;
        }
    }
    let mut answer = 0;
    let mut len = 0;
    for &day in &days {
        if day {
            len += 1;
            answer = answer.max(len);
        } else {
            len = 0;
        }
    }
    println!("{:?}", answer);
}
