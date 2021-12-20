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
    let mut aim: i64 = 0;
    while let Ok((verb, dist)) = parse_input() {
        println!("{}", verb);
        if verb == "forward" {
            horiz += dist;
            depth += aim * dist;
        }
        if verb == "up" {
            aim -= dist;
        }
        if verb == "down" {
            aim += dist;
        }
    }
    println!("{}", horiz * depth);
}
