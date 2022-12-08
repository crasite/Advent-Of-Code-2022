use aoc::Grid;

struct ForestMap {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Grid for ForestMap {
    type Item = u8;

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

fn main() {
    let map = parse(include_str!("input.txt"));
    part_1(&map);
    part_2(&map);
}

fn part_2(map: &ForestMap) {
    let mut max_score = 0;
    for y in 0..map.height() {
        for x in 0..map.width() {
            let mut current_score = 1;
            current_score *= tree_view_direction(map, x as isize, y as isize, (1, 0));
            current_score *= tree_view_direction(map, x as isize, y as isize, (-1, 0));
            current_score *= tree_view_direction(map, x as isize, y as isize, (0, 1));
            current_score *= tree_view_direction(map, x as isize, y as isize, (0, -1));
            if current_score > max_score {
                max_score = current_score;
            }
        }
    }
    println!("part 2: {}", max_score);
}

fn part_1(map: &ForestMap) {
    let mut visible_tree = 0;
    for y in 0..map.height() {
        for x in 0..map.width() {
            if !is_blocked_direction(map, x as isize, y as isize, (1, 0)) {
                visible_tree += 1;
                continue;
            }
            if !is_blocked_direction(map, x as isize, y as isize, (-1, 0)) {
                visible_tree += 1;
                continue;
            }
            if !is_blocked_direction(map, x as isize, y as isize, (0, -1)) {
                visible_tree += 1;
                continue;
            }
            if !is_blocked_direction(map, x as isize, y as isize, (0, 1)) {
                visible_tree += 1;
                continue;
            }
        }
    }
    println!("part 1: {}", visible_tree);
}

fn tree_view_direction(map: &ForestMap, x: isize, y: isize, direction: (isize, isize)) -> u32 {
    let current_value = map.get_from_coordinate(x, y).unwrap();
    let mut x = x;
    let mut y = y;
    let mut score = 0;
    loop {
        x += direction.0;
        y += direction.1;
        if let Some(item) = map.get_from_coordinate(x, y) {
            score += 1;
            if item >= current_value {
                break;
            }
        } else {
            break;
        }
    }
    score
}

fn is_blocked_direction(map: &ForestMap, x: isize, y: isize, direction: (isize, isize)) -> bool {
    let current_value = map.get_from_coordinate(x, y).unwrap();
    let mut x = x;
    let mut y = y;
    loop {
        x += direction.0;
        y += direction.1;
        if let Some(item) = map.get_from_coordinate(x, y) {
            if item >= current_value {
                return true;
            }
        } else {
            return false;
        }
    }
}

fn parse(input: &str) -> ForestMap {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut data: Vec<u8> = Vec::with_capacity(width * height);
    for c in input.chars() {
        if c == '\n' {
            continue;
        }
        data.push(c.to_digit(10).unwrap() as u8);
    }
    ForestMap {
        width,
        height,
        data,
    }
}
