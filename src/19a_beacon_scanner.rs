// advent of code 2021

use anyhow::{Error, Result};
use std::collections::VecDeque;

use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::io::{self, BufRead};
use std::ops;
use text_io::try_scan;
#[macro_use]
extern crate lazy_static;

type Rotation = Box<dyn Fn(&P3) -> P3 + Sync>;
lazy_static! {
    static ref ROTATIONS: Vec<Rotation> = vec![
        Box::new(|P3(x, y, z)| P3(*x, *y, *z)),
        Box::new(|P3(x, y, z)| P3(*x, *z, -y)),
        Box::new(|P3(x, y, z)| P3(*x, -y, -z)),
        Box::new(|P3(x, y, z)| P3(*x, -z, *y)),
        Box::new(|P3(x, y, z)| P3(*y, *x, -z)),
        Box::new(|P3(x, y, z)| P3(*y, *z, *x)),
        Box::new(|P3(x, y, z)| P3(*y, -x, *z)),
        Box::new(|P3(x, y, z)| P3(*y, -z, -x)),
        Box::new(|P3(x, y, z)| P3(*z, *x, *y)),
        Box::new(|P3(x, y, z)| P3(*z, *y, -x)),
        Box::new(|P3(x, y, z)| P3(*z, -x, -y)),
        Box::new(|P3(x, y, z)| P3(*z, -y, *x)),
        Box::new(|P3(x, y, z)| P3(-x, *y, -z)),
        Box::new(|P3(x, y, z)| P3(-x, *z, *y)),
        Box::new(|P3(x, y, z)| P3(-x, -y, *z)),
        Box::new(|P3(x, y, z)| P3(-x, -z, -y)),
        Box::new(|P3(x, y, z)| P3(-y, *x, *z)),
        Box::new(|P3(x, y, z)| P3(-y, *z, -x)),
        Box::new(|P3(x, y, z)| P3(-y, -x, -z)),
        Box::new(|P3(x, y, z)| P3(-y, -z, *x)),
        Box::new(|P3(x, y, z)| P3(-z, *x, -y)),
        Box::new(|P3(x, y, z)| P3(-z, *y, *x)),
        Box::new(|P3(x, y, z)| P3(-z, -x, *y)),
        Box::new(|P3(x, y, z)| P3(-z, -y, -x)),
    ];
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
/// Point in 3D space
struct P3(i32, i32, i32);

impl ops::Add<&P3> for &P3 {
    type Output = P3;

    fn add(self, rhs: &P3) -> P3 {
        P3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<&P3> for &P3 {
    type Output = P3;

    fn sub(self, rhs: &P3) -> P3 {
        P3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

struct Beacon {
    point: P3,
}

struct Scanner {
    beacons: Vec<Beacon>,
    relative_position: RefCell<Option<(P3, &'static Rotation)>>,
    relative_to: Cell<Option<usize>>,
}

fn read_scanner() -> Result<Scanner, Error> {
    let num: i64;
    try_scan!("--- scanner {} ---", num);
    let mut beacons = Vec::new();
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    iterator.next();

    while let Some(Ok(line)) = iterator.next() {
        if line == "" {
            break;
        }
        let coords: Vec<_> = line
            .split(",")
            .map(|s| s.parse::<i32>().map_err(|e| e.into()))
            .collect::<Result<Vec<i32>>>()?;
        assert!(coords.len() == 3);
        beacons.push(Beacon {
            point: P3(coords[0], coords[1], coords[2]),
        });
    }
    Ok(Scanner {
        beacons,
        relative_position: RefCell::new(None),
        relative_to: Cell::new(None),
    })
}

fn main() {
    let mut scanners = Vec::new();
    while let Ok(scanner) = read_scanner() {
        scanners.push(scanner);
    }
    let mut queued: BTreeSet<usize> = BTreeSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<usize> = Vec::new();

    queue.push_back(0);
    queued.insert(0);
    scanners[0]
        .relative_position
        .replace(Some((P3(0, 0, 0), &ROTATIONS[0])));
    scanners[0].relative_to.set(Some(0));
    while let Some(base_index) = queue.pop_front() {
        visited.push(base_index);
        let base = &scanners[base_index];
        let base_point_set: BTreeSet<P3> = base.beacons.iter().map(|b| b.point.clone()).collect();
        for (other_index, other) in scanners.iter().enumerate() {
            if other_index != base_index && !queued.contains(&other_index) {
                'outer: for base_beacon in base.beacons.iter() {
                    for other_beacon in other.beacons.iter() {
                        for rotate in ROTATIONS.iter() {
                            let translation = &rotate(&other_beacon.point) - &base_beacon.point;
                            let mut count = 0;
                            for to_check in other.beacons.iter() {
                                if base_point_set
                                    .contains(&(&rotate(&to_check.point) - &translation))
                                {
                                    count += 1;
                                }
                            }
                            if count >= 12 {
                                other.relative_position.replace(Some((translation, rotate)));
                                other.relative_to.set(Some(base_index));
                                queue.push_back(other_index);
                                queued.insert(other_index);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut beacon_sets: Vec<BTreeSet<P3>> = vec![BTreeSet::new(); scanners.len()];
    for index in visited.iter().rev() {
        let s = &scanners[*index];
        let (translation, rotate) = s.relative_position.take().unwrap();
        let relative_to = s.relative_to.get().unwrap();
        for b in &s.beacons {
            beacon_sets[*index].insert(b.point.clone());
        }
        let translated: Vec<_> = beacon_sets[*index]
            .iter()
            .map(|point| &rotate(point) - &translation)
            .collect();

        beacon_sets[relative_to].extend(translated.into_iter());
    }
    println!("{}", beacon_sets[0].len());
}
