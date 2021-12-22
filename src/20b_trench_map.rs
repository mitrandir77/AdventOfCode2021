// advent of code 2021

use anyhow::{Result};
use itertools::Itertools;
use std::io::{self, BufRead};

const ITERS: usize = 50;

fn into_u8s<'a>(s: &'a String) -> impl Iterator<Item=u8> + 'a {
    s.chars()
        .map(|c| match c {
            '.' =>0,
            '#' =>1,
            _ => panic!("invalid input"),
        })
}

fn apply_algo(algo: &Vec<u8>, map: &Vec<Vec<u8>>, infinity_state: u8) -> (Vec<Vec<u8>>, u8) {
    let get = |x: i64, y: i64| ->  usize{
        if x < 0 || y < 0 {
            return infinity_state as usize;
        }
        let (x, y) = (x as usize, y as usize);
        (*map
            .get(y)
            .map(|row: &Vec<u8>| row.get(x))
            .flatten()
            .unwrap_or(&infinity_state))
        .into()
    };

    let mut new_map= vec![vec![0; map[0].len()]; map.len()];

    for (y, row) in map.iter().enumerate() {
        for (x, _val) in row.iter().enumerate() {
            let index = {
                let (x, y) = (x as i64, y as i64);
                get(x - 1, y - 1) << 8
                    ^ get(x, y - 1) << 7
                    ^ get(x + 1, y - 1) << 6
                    ^ get(x - 1, y) << 5
                    ^ get(x, y) << 4
                    ^ get(x + 1, y) << 3
                    ^ get(x - 1, y + 1) << 2
                    ^ get(x, y + 1) << 1
                    ^ get(x + 1, y + 1)
            };
            new_map[y][x] = algo[index];
        }
    }
    let infinity_state = algo[if infinity_state > 0 {511} else {0}];
    (new_map, infinity_state)
}

fn main() -> Result<()> {
    // PART 1: reading input and constructing the map array
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let algo = into_u8s(&lines.next().unwrap()?).collect_vec();
    assert!(algo.len() == 512);
    assert_eq!(lines.next().unwrap()?, "");
    let map_lines = lines
        .map(|res| {
            res.map(|s| {
                let mut out = vec![0; ITERS];
                out.extend(into_u8s(&s));
                out.extend([0; ITERS]);
                out
            })
            .map_err(|e| e.into())
        })
        .collect::<Result<Vec<Vec<u8>>>>()?;
    let row_len = map_lines[0].len();
    // insert padding above and below
    let mut map = vec![vec![0; row_len]; ITERS];
    map.extend(map_lines.into_iter());
    map.extend(vec![vec![0; row_len]; ITERS]);

    let mut infinity_state = 0;

    for _ in 0..ITERS {
        let (new_map, new_infinity_state) = apply_algo(&algo, &map, infinity_state);
        map = new_map;
        infinity_state = new_infinity_state;
    }

    if infinity_state == 1 {
        println!("infinity");
    }
    else {
        let sum: u64 = map.into_iter().map(|row| row.into_iter().map(|i| i as u64).sum::<u64>()).sum();
        println!("{}", sum);
    }
    Ok(())
}
