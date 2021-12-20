/* advent of code 2021 */
use std::collections::VecDeque;
use text_io::try_read;

fn main() {
    let mut counter = 0;
    let mut window: VecDeque<i64> = VecDeque::new();
    let mut last_sum = None;
    loop {
        if let Ok(depth) = try_read!("{}") {
            window.push_back(depth);
        } else {
            break;
        };
        if window.len() == 3 {
            let sum: i64 = window.iter().sum();
            if let Some(last_sum) = last_sum {
                if sum > last_sum {
                    counter += 1;
                }
            }
            let _pop = window.pop_front().unwrap();
            last_sum = Some(sum);
        }
    }
    println!("{}", counter);
}
