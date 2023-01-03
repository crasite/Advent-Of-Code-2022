use aoc::Grid;

#[derive(Debug)]
struct Map {
    width: u32,
    height: u32,
    data: Vec<bool>,
    is_background_fill: bool,
}
impl Map {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![false; (width * height) as usize],
            is_background_fill: false,
        }
    }

    fn get_total_light(&self) -> u32 {
        self.data.iter().filter(|x| **x).count() as u32
    }
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

    fn display_unit(&self, item: Self::Item) -> char {
        match item {
            true => '#',
            false => '.',
        }
    }
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let command = parse_command(input.next().unwrap());
    assert!(command.len() == 512);
    input.next().unwrap();
    let mut map = parse_map(input);
    for _ in 0..50 {
        let enchanced_map = enchance_map(&map, &command);
        map = enchanced_map;
    }
    dbg!(map.get_total_light());
}

fn enchance_map(map: &Map, command: &[bool]) -> Map {
    let mut enhanced_map = Map::new(map.width + 2, map.height + 2);
    for (idx, current_point) in enhanced_map.data.iter_mut().enumerate() {
        let center_point = Map::index_to_coord_from_width(idx, map.width as usize + 2);
        let center_point = (center_point.0 as isize - 1, center_point.1 as isize - 1);
        let mut total = 0;
        if map
            .get_from_coordinate(center_point.0 - 1, center_point.1 - 1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 8;
        }
        if map
            .get_from_coordinate(center_point.0, center_point.1 - 1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 7;
        }
        if map
            .get_from_coordinate(center_point.0 + 1, center_point.1 - 1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 6;
        }
        if map
            .get_from_coordinate(center_point.0 - 1, center_point.1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 5;
        }
        if map
            .get_from_coordinate(center_point.0, center_point.1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 4;
        }
        if map
            .get_from_coordinate(center_point.0 + 1, center_point.1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 3;
        }
        if map
            .get_from_coordinate(center_point.0 - 1, center_point.1 + 1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 2;
        }
        if map
            .get_from_coordinate(center_point.0, center_point.1 + 1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1 << 1;
        }
        if map
            .get_from_coordinate(center_point.0 + 1, center_point.1 + 1)
            .unwrap_or(map.is_background_fill)
        {
            total += 1;
        }
        *current_point = command[total];
    }
    if map.is_background_fill {
        enhanced_map.is_background_fill = *command.last().unwrap();
    } else {
        enhanced_map.is_background_fill = *command.first().unwrap();
    }
    enhanced_map
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
