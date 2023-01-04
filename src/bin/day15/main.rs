use std::collections::HashSet;

use aoc::prelude::*;

type Coord = (isize, isize);
#[derive(Debug)]
struct Beacon {
    coord: Coord,
    closest: Coord,
}

fn main() {
    let input = include_str!("input.txt");
    let re = regex::Regex::new(
        r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
    )
    .unwrap();
    part_1(input, &re, 10);
    part_2(input, &re, 4000000);
}

fn part_2(input: &str, re: &regex::Regex, limit: isize) {
    let mut beacons = Vec::new();
    let tuning_amplifier = 4000000;
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        beacons.push(Beacon {
            coord: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            closest: (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        });
    }
    for level in 0..=limit {
        let mut ranges = Vec::new();
        for beacon in beacons.iter() {
            let distance = beacon.coord.0.abs_diff(beacon.closest.0)
                + beacon.coord.1.abs_diff(beacon.closest.1);
            let offset = beacon.coord.1.abs_diff(level);
            if offset > distance {
                continue;
            }
            ranges.push((
                beacon.coord.0 - (distance - offset) as isize,
                beacon.coord.0 + (distance - offset) as isize,
            ));
        }
        ranges.sort_by_key(|range| range.0);
        let final_range = ranges
            .into_iter()
            .coalesce(|a, b| {
                if a.1 >= b.0 {
                    Ok((a.0, a.1.max(b.1)))
                } else {
                    Err((a, b))
                }
            })
            .filter_map(|v| {
                if v.1 < 0 || v.0 > limit {
                    None
                } else {
                    Some((0.max(v.0), limit.min(v.1)))
                }
            })
            .collect::<Vec<_>>();
        let count = final_range.iter().fold(0, |acc, n| acc + (n.1 - n.0 + 1));
        if count < limit + 1 {
            println!(
                "Part 2: {}",
                level + ((final_range[0].1 + 1) * tuning_amplifier)
            );
            break;
        }
    }
}

fn part_1(input: &str, re: &regex::Regex, level: isize) {
    let mut beacons = Vec::new();
    let mut ranges = Vec::new();
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        beacons.push(Beacon {
            coord: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            closest: (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        });
    }
    for beacon in beacons.iter() {
        let distance =
            beacon.coord.0.abs_diff(beacon.closest.0) + beacon.coord.1.abs_diff(beacon.closest.1);
        let offset = beacon.coord.1.abs_diff(level);
        if offset > distance {
            continue;
        }
        ranges.push((
            beacon.coord.0 - (distance - offset) as isize,
            beacon.coord.0 + (distance - offset) as isize,
        ));
    }
    ranges.sort_by_key(|range| range.0);
    let final_range = ranges
        .into_iter()
        .coalesce(|a, b| {
            if a.1 >= b.0 {
                Ok((a.0, a.1.max(b.1)))
            } else {
                Err((a, b))
            }
        })
        .collect::<Vec<_>>();
    let mut in_range = HashSet::new();
    for beacon in beacons.iter() {
        if beacon.closest.1 != level {
            continue;
        }
        let Some(beacon) = final_range
            .iter()
            .find(|range| beacon.closest.0 >= range.0 && beacon.closest.0 <= range.1) else {
                continue;
            };
        in_range.insert(beacon);
    }
    let count = final_range.iter().fold(0, |acc, n| acc + (n.1 - n.0 + 1));
    println!("Part 1: {}", count as usize - in_range.len());
}
