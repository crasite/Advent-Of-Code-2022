use aoc::prelude::*;
#[derive(Debug, PartialEq, Eq)]
struct Packet {
    data: String,
}
impl Packet {
    fn get_subpacket(&self) -> Vec<String> {
        let mut subpackets = vec![];
        let mut current_packet = String::new();
        let mut data = self.data.clone();
        let mut current_bracket = 0;
        data.pop().unwrap();
        data.remove(0);
        for c in data.chars() {
            match c {
                '[' => {
                    current_bracket += 1;
                    current_packet.push(c);
                }
                ']' => {
                    current_bracket -= 1;
                    current_packet.push(c);
                }
                ',' => {
                    if current_bracket == 0 {
                        subpackets.push(current_packet);
                        current_packet = String::new();
                    } else {
                        current_packet.push(c);
                    }
                }
                _ => {
                    current_packet.push(c);
                }
            };
        }
        if !current_packet.is_empty() {
            subpackets.push(current_packet);
        }
        subpackets
    }

    fn is_valid(&self, other: &Self) -> Option<bool> {
        let sub_packets = self.get_subpacket();
        let other_sub_packets = other.get_subpacket();
        if sub_packets.is_empty() && !other_sub_packets.is_empty() {
            return Some(true);
        }
        for (idx, sub_packet) in sub_packets.iter().enumerate() {
            if other_sub_packets.get(idx).is_none() {
                return Some(false);
            } else if sub_packet.starts_with('[')
                || other_sub_packets.get(idx).unwrap().starts_with('[')
            {
                let other = other_sub_packets.get(idx).unwrap();
                let rs = convert_to_packet(sub_packet).is_valid(&convert_to_packet(other));
                if rs.is_some() {
                    return rs;
                } else {
                    continue;
                }
            } else {
                let other_val = other_sub_packets.get(idx).unwrap().parse::<u32>().unwrap();
                let self_val = sub_packet.parse::<u32>().unwrap();
                match self_val.cmp(&other_val) {
                    std::cmp::Ordering::Less => return Some(true),
                    std::cmp::Ordering::Greater => return Some(false),
                    _ => continue,
                }
            }
        }
        if sub_packets.len() < other_sub_packets.len() {
            return Some(true);
        }
        None
    }
}

fn convert_to_packet(input: &str) -> Packet {
    if input.starts_with('[') {
        Packet {
            data: input.to_string(),
        }
    } else {
        let mut data = input.to_string();
        data.push(']');
        data.insert(0, '[');
        Packet {
            data: data.to_string(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(validity) = self.is_valid(other) {
            if validity {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Greater)
            }
        } else {
            None
        }
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
    Ok(())
}

fn part2(input: &str) {
    let mut input_line = input.lines();
    let mut packet_list = vec![];
    let dk1 = "[[2]]";
    let dk2 = "[[6]]";
    packet_list.push(Packet { data: dk1.into() });
    packet_list.push(Packet { data: dk2.into() });
    loop {
        let p1 = Packet {
            data: input_line.next().unwrap().to_string(),
        };
        let p2 = Packet {
            data: input_line.next().unwrap().to_string(),
        };
        packet_list.push(p1);
        packet_list.push(p2);
        if input_line.next().is_none() {
            break;
        }
    }
    packet_list.sort();
    let Some((idx1,_))=packet_list
        .iter()
        .enumerate()
        .find(|(_idx, p)| p.data == dk1) else {
        panic!("Not found");
        };
    let Some((idx2,_))=packet_list
        .iter()
        .enumerate()
        .find(|(_idx, p)| p.data == dk2) else {
        panic!("Not found");
        };
    println!("Part 2:{}", (idx1 + 1) * (idx2 + 1));
}
fn part1(input: &str) {
    let mut input_line = input.lines();
    let mut i = 1;
    let mut rs = 0;
    loop {
        let p1 = Packet {
            data: input_line.next().unwrap().to_string(),
        };
        let p2 = Packet {
            data: input_line.next().unwrap().to_string(),
        };
        // dbg!(p1.get_subpacket(), p2.get_subpacket());
        if p1.is_valid(&p2).unwrap() {
            rs += i;
        }
        i += 1;
        if input_line.next().is_none() {
            break;
        }
    }
    println!("Part 1: {}", rs);
}
