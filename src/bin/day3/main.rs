use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

fn part_two(input: &str) {
    let mut total_priorities: u32 = 0;
    let mut line = input.lines();
    loop {
        let Some(line1) =  line.next() else {
          break;
        };
        let set1 = line_to_map(line1);
        let set2 = line_to_map(line.next().unwrap());
        let set3 = line_to_map(line.next().unwrap());
        for c in set1 {
            if set2.contains(&c) && set3.contains(&c) {
                total_priorities += char_to_value(&c.to_string());
                break;
            }
        }
    }
    println!("Total Priorities: {}", total_priorities);
}

fn part_one(input: &str) {
    let mut total_prioritiies: u32 = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        for c in left.chars() {
            if right.find(c).is_some() {
                total_prioritiies += char_to_value(&c.to_string());
                break;
            }
        }
    }
    println!("Total Priorities: {}", total_prioritiies);
}

fn char_to_value(c: &str) -> u32 {
    let mut rs = c.as_bytes()[0] as u32;
    if rs <= 90 {
        rs -= 38;
    } else {
        rs -= 96;
    }
    rs
}

fn line_to_map(line: &str) -> HashSet<char> {
    let mut hm = HashSet::new();
    for c in line.chars() {
        hm.insert(c);
    }
    hm
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(char_to_value("a"), 1);
        assert_eq!(char_to_value("b"), 2);
        assert_eq!(char_to_value("z"), 26);
        assert_eq!(char_to_value("A"), 27);
        assert_eq!(char_to_value("B"), 28);
        assert_eq!(char_to_value("Z"), 52);
    }
}
