/* advent of code 2021 */
use anyhow::Error;
use text_io::try_scan;

fn parse_input() -> Result<(String, i64), Error> {
    let verb: String;
    let dist: i64;
    try_scan!("{} {}", verb, dist);
    Ok((verb, dist))
}

fn main() {
    let mut horiz: i64 = 0;
    let mut depth: i64 = 0;
    loop {
        if let Ok((verb, dist)) = parse_input() {
            println!("{}", verb);
            if verb == "forward" {
                horiz = horiz + dist;
            }
            if verb == "up" {
                depth = depth - dist;
            }
            if verb == "down" {
                depth = depth + dist;
            }
        } else {
            break;
        }
    }
    println!("{}", horiz * depth);
}
