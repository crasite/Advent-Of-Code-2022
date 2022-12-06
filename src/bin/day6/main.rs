use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    part_1(input);
    part_2(input);
}

fn part_2(input: &str) {
    let mut packets = VecDeque::new();
    for (idx, c) in input.char_indices() {
        if packets.len() < 14 {
            packets.push_front(c);
            continue;
        } else {
            packets.push_front(c);
            packets.pop_back();
            if is_unique(packets.make_contiguous(), 14) {
                println!("{}", idx + 1);
                break;
            }
        }
    }
}
fn part_1(input: &str) {
    let mut packets = VecDeque::new();
    for (idx, c) in input.char_indices() {
        if packets.len() < 4 {
            packets.push_front(c);
            continue;
        } else {
            packets.push_front(c);
            packets.pop_back();
            if is_unique(packets.make_contiguous(), 4) {
                println!("{}", idx + 1);
                break;
            }
        }
    }
}

fn is_unique(packets: &[char], len: usize) -> bool {
    let mut set = HashSet::new();
    assert!(packets.len() == len);
    for packet in packets {
        if !set.insert(packet) {
            return false;
        }
    }
    true
}
