#[derive(Debug)]
struct Command<'a> {
    op: &'a str,
    val: Option<isize>,
}

fn main() {
    let commands = parse(include_str!("input.txt"));
    run_commands(&commands, 20, 40);
}

fn parse(input: &str) -> Vec<Command> {
    let mut commands = vec![];
    for line in input.lines() {
        let mut split = line.split(' ');
        let op = split.next().unwrap();
        let val = split.next().map(|v| v.parse().unwrap());
        commands.push(Command { op, val })
    }
    commands
}

fn run_commands(
    commands: &[Command],
    cycle_stop_starting_point: isize,
    cycle_stop_interval: isize,
) {
    let mut register_x = 1;
    let mut cycle: isize = 0;
    let mut strength: isize = 0;
    for command in commands {
        if cycle > 220 {
            break;
        }
        match command.op {
            "noop" => {
                if cycle + 1 == cycle_stop_starting_point {
                    strength += dbg!((cycle + 1)) * dbg!(register_x);
                }
                if cycle > cycle_stop_starting_point
                    && (cycle + 1 - cycle_stop_starting_point) % cycle_stop_interval == 0
                {
                    strength += dbg!((cycle + 1)) * dbg!(register_x);
                }
                cycle += 1;
            }
            "addx" => {
                if cycle + 1 == cycle_stop_starting_point || cycle + 2 == cycle_stop_starting_point
                {
                    let cycle = if (cycle + 2) % cycle_stop_starting_point == 0 {
                        cycle + 2
                    } else {
                        cycle + 1
                    };
                    strength += dbg!(cycle) * dbg!(register_x);
                }
                if cycle > cycle_stop_starting_point
                    && (cycle + 2 - cycle_stop_starting_point) % cycle_stop_interval <= 1
                {
                    let cycle =
                        if (cycle + 2 - cycle_stop_starting_point) % cycle_stop_interval == 0 {
                            cycle + 2
                        } else {
                            cycle + 1
                        };
                    strength += dbg!(cycle) * dbg!(register_x);
                }
                register_x += command.val.unwrap();
                cycle += 2;
            }
            _ => unreachable!("Unknown Commands"),
        }
    }
    println!("Cycle: {}", cycle);
    println!("Part 1: {}", strength);
}
