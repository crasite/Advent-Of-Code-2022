use aoc::Grid;

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
    data: Vec<bool>,
    is_background_fill: bool,
}

impl Grid for Map {
    type Item = bool;

    fn width(&self) -> usize {
        self.width as usize
    }

    fn height(&self) -> usize {
        self.height as usize
    }

    fn get_at_index(&self, index: usize) -> Option<Self::Item> {
        self.data.get(index).copied()
    }
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let command = parse_command(input.next().unwrap());
    assert!(command.len() == 512);
    input.next().unwrap();
    let map = parse_map(input);
    dbg!(map
        .get_from_coordinate(-1, -1)
        .unwrap_or(map.is_background_fill));
    dbg!(map.get_from_coordinate(1, 0));
    dbg!(map.get_from_coordinate(2, 1));
}

fn parse_command(input: &str) -> Vec<bool> {
    let mut command = vec![];
    for c in input.chars() {
        match c {
            '.' => command.push(false),
            '#' => command.push(true),
            _ => unreachable!(),
        }
    }
    command
}

fn parse_map<'a>(input: impl Iterator<Item = &'a str>) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut data = vec![];
    for line in input {
        width = line.len() as u32;
        for c in line.chars() {
            data.push(c == '#');
        }
        height += 1;
    }
    Map {
        width,
        height,
        data,
        is_background_fill: false,
    }
}
