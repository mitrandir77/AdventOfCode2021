/* advent of code 2021 */
#![feature(btree_drain_filter)]
use std::collections::BTreeSet;

use anyhow::Error;
use std::io::{self, BufRead};
fn find_oxygen(numbers: &mut BTreeSet<&str>, i: usize) -> String {
    let mut cnt0 = 0;
    let mut cnt1 = 0;
    for num in numbers.iter() {
        if num.len() == i {
            return num.to_string();
        }
        if num.as_bytes()[i] == '0' as u8 {
            cnt0 += 1;
        } else {
            cnt1 += 1;
        }
    }
    let filter_char = if cnt1 >= cnt0 { b'1' } else { b'0' };
    numbers.drain_filter(|v| v.as_bytes()[i] != filter_char);
    find_oxygen(numbers, i + 1)
}
fn find_co2(numbers: &mut BTreeSet<&str>, i: usize) -> String {
    let mut cnt0 = 0;
    let mut cnt1 = 0;
    for num in numbers.iter() {
        if num.len() == i {
            return num.to_string();
        }
        if num.as_bytes()[i] == b'0' {
            cnt0 += 1;
        } else {
            cnt1 += 1;
        }
    }
    let filter_char = if cnt1 == 0 || cnt0 != 0 && cnt1 >= cnt0 {
        b'0'
    } else {
        b'1'
    };
    numbers.drain_filter(|v| v.as_bytes()[i] != filter_char);
    find_co2(numbers, i + 1)
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut numbers = BTreeSet::new();
    for line in stdin.lock().lines() {
        let num = line?;
        numbers.insert(num);
    }
    let oxygen = find_oxygen(&mut numbers.iter().map(|v| v.as_str()).collect(), 0);
    let co2 = find_co2(&mut numbers.iter().map(|v| v.as_str()).collect(), 0);
    let oxygen = usize::from_str_radix(&oxygen, 2)?;
    let co2 = usize::from_str_radix(&co2, 2)?;
    println!("{}", oxygen * co2);
    Ok(())
}
