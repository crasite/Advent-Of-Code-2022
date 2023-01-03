use std::collections::HashMap;

use aoc::prelude::*;

type Coord = (i32, i32);

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let drop_point = (500, 0);
    part_1(input, drop_point);
    part_2(input, drop_point);
    Ok(())
}

fn part_1(input: &str, drop_point: (i32, i32)) {
    let mut map = parse(input);
    let max_y = map.keys().max_by_key(|(_, y)| y).unwrap().1;
    simulate(drop_point, &mut map, max_y);
    let total = map
        .iter()
        .fold(0, |acc, (_, v)| if *v == 'o' { acc + 1 } else { acc });
    println!("Part 1: {}", total);
}
fn part_2(input: &str, drop_point: (i32, i32)) {
    let mut map = parse(input);
    let max_y = map.keys().max_by_key(|(_, y)| y).unwrap().1;
    simulate2(drop_point, &mut map, max_y);
    let total = map
        .iter()
        .fold(0, |acc, (_, v)| if *v == 'o' { acc + 1 } else { acc });
    println!("Part 2: {}", total);
}

fn simulate(drop_point: (i32, i32), map: &mut HashMap<(i32, i32), char>, max_y: i32) {
    let mut droplet = drop_point;
    loop {
        let below = (droplet.0, droplet.1 + 1);
        if map.get(&below).is_some() {
            if map.get(&(below.0 - 1, below.1)).is_none() {
                droplet = (below.0 - 1, below.1);
            } else if map.get(&(below.0 + 1, below.1)).is_none() {
                droplet = (below.0 + 1, below.1);
            } else {
                map.insert(droplet, 'o');
                droplet = drop_point;
            }
        } else if droplet.1 > max_y {
            break;
        } else {
            droplet = below
        }
    }
}

fn simulate2(drop_point: (i32, i32), map: &mut HashMap<(i32, i32), char>, max_y: i32) {
    let mut droplet = drop_point;
    loop {
        let below = (droplet.0, droplet.1 + 1);
        if map.get(&below).is_some() {
            if map.get(&(below.0 - 1, below.1)).is_none() {
                droplet = (below.0 - 1, below.1);
            } else if map.get(&(below.0 + 1, below.1)).is_none() {
                droplet = (below.0 + 1, below.1);
            } else {
                if droplet == drop_point {
                    map.insert(droplet, 'o');
                    break;
                }
                map.insert(droplet, 'o');
                droplet = drop_point;
            }
        } else if droplet.1 > max_y {
            map.insert(droplet, 'o');
            droplet = drop_point;
        } else {
            droplet = below
        }
    }
}
#[allow(dead_code)]
fn draw(map: &HashMap<(i32, i32), char>) {
    let min_x = map.keys().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = map.keys().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = map.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = map.keys().max_by_key(|(_, y)| y).unwrap().1;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", map.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
    println!();
}

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let map = input
        .lines()
        .fold(HashMap::new(), |mut acc: HashMap<Coord, char>, n| {
            let mut coord_origin = None;
            for coord_raw in n.split(" -> ") {
                let coord = coord_raw
                    .split(',')
                    .into_iter()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                insert_into_map(&mut coord_origin, coord, &mut acc);
            }
            acc
        });
    map
}

fn insert_into_map(
    coord_origin: &mut Option<(i32, i32)>,
    coord: Vec<i32>,
    acc: &mut HashMap<(i32, i32), char>,
) {
    match *coord_origin {
        None => {}
        Some(coord_origin) => {
            let min_x = coord_origin.0.min(coord[0]);
            let max_x = coord_origin.0.max(coord[0]);
            let min_y = coord_origin.1.min(coord[1]);
            let max_y = coord_origin.1.max(coord[1]);
            for x in min_x..=max_x {
                acc.insert((x, coord[1]), '#');
            }
            for y in min_y..=max_y {
                acc.insert((coord[0], y), '#');
            }
        }
    }

    *coord_origin = Some((coord[0], coord[1]));
}
