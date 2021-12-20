/* advent of code 2021 */
use std::cmp::max;

use text_io::scan;

fn main() {
    let (tx1, tx2, ty1, ty2): (i64, i64, i64, i64);
    scan!("target area: x={}..{}, y={}..{}", tx1, tx2, ty1, ty2);

    let check_velocity = |mut vx: i64, mut vy: i64| -> bool {
        let (mut x, mut y) = (0, 0);
        while x <= tx2 && y >= ty1 {
            if x >= tx1 && x <= tx2 && y >= ty1 && y <= ty2 {
                return true;
            }
            x = x + vx;
            y = y + vy;
            vy = vy - 1;
            vx = max(vx - 1, 0);
        }
        return false;
    };

    let mut count: i64 = 0;
    for vx in 0..=tx2 {
        for vy in (-ty1.abs())..=(ty1.abs()) {
            if check_velocity(vx, vy) {
                count = count + 1;
            }
        }
    }

    println!("{}", count);
}
