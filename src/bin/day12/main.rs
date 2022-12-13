use std::collections::HashMap;

use aoc::{Grid, PathFindable};

struct MountainMap {
    width: usize,
    height: usize,
    data: Vec<char>,
    starting: usize,
    ending: usize,
}
impl Grid for MountainMap {
    type Item = char;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_at_index(&self, index: usize) -> Option<Self::Item> {
        self.data.get(index).copied()
    }

    fn display_unit(&self, item: Self::Item) -> char {
        format!("{}", item).chars().next().unwrap()
    }
}

impl PathFindable for MountainMap {
    fn movement_cost(&self, from: (&isize, &isize), to: (&isize, &isize)) -> Option<usize> {
        let Some(target) = self.get_from_coordinate(*to.0, *to.1) else {
            return None;
        };
        let Some(starting) = self.get_from_coordinate(*from.0, *from.1) else {
            return None;
        };
        if target == 'S' {
            None
        } else if target == 'E' {
            if starting as u32 + 1 >= 'z' as u32 {
                Some((from.0 - to.0).unsigned_abs() + (from.1 - to.1).unsigned_abs())
            } else {
                None
            }
        } else if starting as u32 + 1 >= target as u32 || starting == 'S' {
            Some((from.0 - to.0).unsigned_abs() + (from.1 - to.1).unsigned_abs())
        } else {
            None
        }
    }

    fn h_value(&self, from: (&isize, &isize), to: (&isize, &isize)) -> Option<usize> {
        Some((from.0 - to.0).unsigned_abs() + (from.1 - to.1).unsigned_abs())
    }
}

impl MountainMap {
    fn shortest_distance_plain(&self, to: (&isize, &isize)) -> Option<usize> {
        #[derive(Clone, Hash, Eq, PartialEq, Debug)]
        struct Node {
            x: isize,
            y: isize,
            f: usize,
            g: usize,
            from: Option<(isize, isize)>,
        }
        let mut open_set = HashMap::new();
        let mut close_set: HashMap<(isize, isize), Node> = HashMap::new();

        let mut starting_pos = vec![];
        for i in 0..self.data.len() {
            if self.get_from_index(i) == Some('a') || self.get_from_index(i) == Some('S') {
                starting_pos.push(i);
            }
        }
        starting_pos.iter().for_each(|pos| {
            let coord = self.index_to_coord(*pos);
            let Some(starting_h) = self.h_value((&coord.0,&coord.1), to) else {
                panic!("Invalid starting position")
            };
            let starting_node = Node {
                x: coord.0,
                y: coord.1,
                f: starting_h,
                g: 0,
                from: None,
            };
            open_set.insert((coord.0, coord.1), starting_node);
        });
        while !open_set.is_empty() {
            if let Some(target) = close_set.get(&(*to.0, *to.1)) {
                return Some(target.g);
            }
            let n = open_set
                .iter()
                .reduce(|acc, i| if acc.1.f < i.1.f { acc } else { i });
            let entry = n.unwrap();
            let node = entry.1.clone();

            assert!(open_set.remove(&entry.0.clone()).is_some());
            close_set.insert((node.x, node.y), node.clone());
            let neighbors = self.get_neighbors(node.x, node.y);
            for neighbor in neighbors {
                let g = self
                    .movement_cost((&node.x, &node.y), (&neighbor.0, &neighbor.1))
                    .unwrap()
                    + node.g;
                if let Some(existing_node) = open_set
                    .get(&(neighbor.0, neighbor.1))
                    .or_else(|| close_set.get(&(neighbor.0, neighbor.1)))
                {
                    if existing_node.g <= g {
                        continue;
                    } else {
                        open_set.remove(&(neighbor.0, neighbor.1));
                        close_set.remove(&(neighbor.0, neighbor.1));
                    }
                }
                let h = self.h_value((&neighbor.0, &neighbor.1), to).unwrap();
                let f = g + h;
                open_set.insert(
                    (neighbor.0, neighbor.1),
                    Node {
                        x: neighbor.0,
                        y: neighbor.1,
                        f,
                        g,
                        from: Some((node.x, node.y)),
                    },
                );
            }
        }

        if let Some(target) = close_set.get(&(*to.0, *to.1)) {
            return Some(target.g);
        }
        None
    }
}

fn main() {
    let map = parse(include_str!("input.txt"));
    part_1(&map);
    part_2(&map);
}

fn part_2(map: &MountainMap) {
    let to = map.index_to_coord(map.ending);
    let lowest_score = map.shortest_distance_plain((&to.0, &to.1));
    println!("Part 2: {}", lowest_score.unwrap());
}

fn part_1(map: &MountainMap) {
    let from = map.index_to_coord(map.starting);
    let to = map.index_to_coord(map.ending);
    let st = map.shortest_distance_bf((&from.0, &from.1), (&to.0, &to.1));
    println!("Part 1: {:?}", st.unwrap());
}

fn parse(input: &str) -> MountainMap {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut data: Vec<char> = Vec::with_capacity(width * height);
    let mut starting = None;
    let mut ending = None;
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        if c == 'S' {
            starting = Some(data.len());
        } else if c == 'E' {
            ending = Some(data.len());
        }
        data.push(c);
    }
    MountainMap {
        width,
        height,
        data,
        starting: starting.unwrap(),
        ending: ending.unwrap(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn chart() {
        let a = 'a' as u32;
        let b = 'b' as u32;
        assert!(a + 1 == b)
    }
}
