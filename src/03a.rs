use std::collections::BTreeMap;

/* advent of code 2021 */
use anyhow::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Error>{
    let mut cnt0 = BTreeMap::new();
    let mut cnt1 = BTreeMap::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let num = line?;
        for (i, c) in num.chars().enumerate() {
            if c == '0' {
                *cnt0.entry(i).or_insert(0) += 1;
            }
            else {
                *cnt1.entry(i).or_insert(0) += 1;
            }
        }
    }
    let mut gamma = 0; 
    let mut epsilon = 0;
    for k in cnt0.keys() {
        gamma *= 2;
        epsilon *= 2;
        if cnt0[k] < cnt1[k] {
            gamma += 1;
        }
        else {
            epsilon += 1;
        }
    }
    println!("{}", gamma * epsilon);
    Ok(())
}
