use std::{cmp::Ordering, collections::HashSet};

type Pos = (isize, isize);

fn main() {
    let input = include_str!("input.txt");
    part_1(input);
    part_2(input)
}
fn part_2(input: &str) {
    let mut rope: Vec<Pos> = Vec::new();
    let rope_len = 10;
    for _ in 0..rope_len {
        rope.push((0, 0));
    }
    let mut visited: HashSet<Pos> = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let count = split.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..count {
            match direction {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => unreachable!("Invalid direction"),
            }
            for i in 1..rope_len {
                let (head, tail) = rope.split_at_mut(i);
                move_tail(tail.first_mut().unwrap(), head.last().unwrap());
                if i == rope_len - 1 {
                    visited.insert(*tail.first().unwrap());
                }
            }
        }
    }
    println!("Part 1: {}", visited.len());
}

fn part_1(input: &str) {
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);
    let mut visited: HashSet<Pos> = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap();
        let count = split.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..count {
            match direction {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => unreachable!("Invalid direction"),
            }
            move_tail(&mut tail, &head);
            visited.insert(tail);
        }
    }
    println!("Part 1: {}", visited.len());
}

fn move_tail(tail: &mut Pos, head: &Pos) {
    let (hx, hy) = head;
    let (tx, ty) = tail;
    if !((hx - *tx).abs() <= 1 && (hy - *ty).abs() <= 1) {
        match hx.cmp(tx) {
            Ordering::Greater => tail.0 += 1,
            Ordering::Less => tail.0 -= 1,
            _ => {}
        }
        match hy.cmp(ty) {
            Ordering::Greater => tail.1 += 1,
            Ordering::Less => tail.1 -= 1,
            _ => {}
        }
    }
}
