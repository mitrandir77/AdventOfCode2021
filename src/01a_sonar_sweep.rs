/* advent of code 2021 */
use text_io::try_read;

fn main() {
    let mut counter = 0;
    let mut last_depth = None;
    loop {
        let depth: i64 = if let Ok(depth) = try_read!("{}") {
            depth
        } else {
            break;
        };
        if let Some(last_depth) = last_depth {
            if depth > last_depth {
                counter = counter + 1;
            }
        }
        last_depth = Some(depth);
    }
    println!("{}", counter);
}
