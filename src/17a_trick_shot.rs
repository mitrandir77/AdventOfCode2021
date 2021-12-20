/* advent of code 2021 */
use text_io::scan;

fn main() {
    let (tx1, tx2, ty1, ty2): (i64, i64, i64, i64);
    scan!("target area: x={}..{}, y={}..{}", tx1, tx2, ty1, ty2);
    let vy = -ty1 - 1;
    let highy = (vy * (vy + 1)) / 2;
    println!("{}", highy);
}
