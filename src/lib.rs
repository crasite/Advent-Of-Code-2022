use std::{collections::HashMap, fmt::Display};
pub mod prelude {
    pub use anyhow::{anyhow, bail, Result};
    pub use tracing::{debug, error, info, trace, warn};
}

pub trait Grid {
    type Item: Display;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_at_index(&self, index: usize) -> Option<Self::Item>;
    fn index_to_coord_from_width(index: usize, width: usize) -> (usize, usize) {
        let x = index % width;
        let y = index / width;
        (x, y)
    }
    fn index_to_coord(&self, index: usize) -> (isize, isize) {
        let width = self.width();
        let x = index % width;
        let y = index / width;
        (x as isize, y as isize)
    }
    fn get_from_coordinate(&self, x: isize, y: isize) -> Option<Self::Item> {
        if x < 0 || y < 0 || x >= self.width() as isize || y >= self.height() as isize {
            None
        } else {
            self.get_at_index(y as usize * self.width() + x as usize)
        }
    }
    fn get_from_index(&self, index: usize) -> Option<Self::Item> {
        self.get_at_index(index)
    }

    fn display_unit(&self, item: Self::Item) -> char;

    fn draw(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!(
                    "{}",
                    self.display_unit(self.get_from_coordinate(x as isize, y as isize).unwrap())
                );
            }
            println!();
        }
        println!();
    }
}

pub trait PathFindable: Grid {
    fn movement_cost(&self, from: (&isize, &isize), to: (&isize, &isize)) -> Option<usize>;
    fn h_value(&self, from: (&isize, &isize), to: (&isize, &isize)) -> Option<usize>;

    fn get_neighbors(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let mut neighbors = vec![];
        if self.movement_cost((&x, &y), (&(x - 1), &y)).is_some() {
            neighbors.push((x - 1, y));
        }
        if self.movement_cost((&x, &y), (&(x + 1), &y)).is_some() {
            neighbors.push((x + 1, y));
        }
        if self.movement_cost((&x, &y), (&x, &(y + 1))).is_some() {
            neighbors.push((x, y + 1));
        }
        if self.movement_cost((&x, &y), (&x, &(y - 1))).is_some() {
            neighbors.push((x, y - 1));
        }
        neighbors
    }

    fn shortest_distance_bf(&self, from: (&isize, &isize), to: (&isize, &isize)) -> Option<usize> {
        #[derive(Clone, Hash, Eq, PartialEq, Debug)]
        struct Node {
            x: isize,
            y: isize,
            f: usize,
            g: usize,
            from: Option<(isize, isize)>,
        }
        let Some(starting_h) = self.h_value(from, to) else {
            return None;
        };
        let starting_node = Node {
            x: *from.0,
            y: *from.1,
            f: starting_h,
            g: 0,
            from: None,
        };
        let mut open_set = HashMap::new();
        let mut close_set: HashMap<(isize, isize), Node> = HashMap::new();
        open_set.insert((starting_node.x, starting_node.y), starting_node);
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
