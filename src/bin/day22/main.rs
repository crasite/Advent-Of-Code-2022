#[derive(Debug)]
#[allow(dead_code)]
struct Command {
    is_on: bool,
    x_bound: (isize, isize),
    y_bound: (isize, isize),
    z_bound: (isize, isize),
}

fn main() {
    #[allow(unused_variables)]
    let commands = dbg!(parse(include_str!("input.txt")));
}

fn parse(input: &str) -> Vec<Command> {
    let mut rs = vec![];
    for line in input.lines() {
        let mut split = line.split(' ');
        let is_on = match split.next().unwrap() {
            "on" => true,
            "off" => false,
            s => unreachable!("Unknown command: {}", s),
        };
        let mut split_bounds = split.next().unwrap().split(',');
        let mut x_bound_split = split_bounds.next().unwrap().split('=');
        let mut x_bound_split = x_bound_split.nth(1).unwrap().split("..");
        let x_bound: (isize, isize) = (
            x_bound_split.next().unwrap().parse().unwrap(),
            x_bound_split.next().unwrap().parse().unwrap(),
        );
        let mut y_bound_split = split_bounds.next().unwrap().split('=');
        let mut y_bound_split = y_bound_split.nth(1).unwrap().split("..");
        let y_bound: (isize, isize) = (
            y_bound_split.next().unwrap().parse().unwrap(),
            y_bound_split.next().unwrap().parse().unwrap(),
        );
        let mut z_bound_split = split_bounds.next().unwrap().split('=');
        let mut z_bound_split = z_bound_split.nth(1).unwrap().split("..");
        let z_bound: (isize, isize) = (
            z_bound_split.next().unwrap().parse().unwrap(),
            z_bound_split.next().unwrap().parse().unwrap(),
        );
        rs.push(Command {
            is_on,
            x_bound,
            y_bound,
            z_bound,
        })
    }
    rs
}
