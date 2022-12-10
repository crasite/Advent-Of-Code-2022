#[derive(Debug, Clone)]
struct Command<'a> {
    op: &'a str,
    val: Option<isize>,
}

#[derive(Debug, Clone)]
struct State<'a> {
    commands: &'a [Command<'a>],
    current_cycle: isize,
    x_register: isize,
    pending_cycle: usize,
    pending_command: &'a Command<'a>,
}

fn main() {
    let commands = parse(include_str!("input.txt"));
    let starting_state = State {
        commands: &commands,
        current_cycle: 0,
        x_register: 1,
        pending_cycle: 0,
        pending_command: &Command {
            op: "noop",
            val: None,
        },
    };
    part_1(&starting_state, 220, 0);
    println!("Part 2:");
    part_2(&run(&starting_state), 0, String::new());
}

fn part_2(starting_state: &State, mut sprite_pos: isize, mut current_line: String) {
    if sprite_pos == 40 {
        println!("{}", current_line);
        current_line.clear();
        sprite_pos = 0;
    }

    if (sprite_pos - starting_state.x_register).abs() <= 1 {
        current_line.push('#');
    } else {
        current_line.push('.');
    }
    if starting_state.commands.is_empty() {
        println!("{}", current_line);
        return;
    }
    let tmpstate = run(starting_state);
    part_2(&tmpstate, sprite_pos + 1, current_line);
}

fn part_1(starting_state: &State, count: usize, strength: isize) {
    let mut new_strength = strength;
    if [20, 60, 100, 140, 180, 220].contains(&starting_state.current_cycle) {
        new_strength += starting_state.current_cycle * starting_state.x_register;
    }
    if count == 0 {
        println!("Part 1: {new_strength}");
        return;
    }
    let tmpstate = run(starting_state);
    part_1(&tmpstate, count - 1, new_strength);
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

fn run<'a>(state: &'a State) -> State<'a> {
    if state.pending_cycle > 0 {
        let mut new_state = state.clone();
        new_state.current_cycle += 1;
        new_state.pending_cycle -= 1;
        new_state
    } else {
        let mut new_state = state.clone();
        new_state.commands = state.commands.split_at(1).1;
        new_state.current_cycle += 1;
        new_state.x_register = calculate_pending_command(state);
        if state.commands.first().is_none() {
            return new_state;
        }
        let cmd = state.commands.first().unwrap();
        calculate_new_state(cmd, new_state)
    }
}

fn calculate_new_state<'a>(cmd: &'a Command, mut new_state: State<'a>) -> State<'a> {
    match cmd.op {
        "noop" => {
            new_state.pending_command = cmd;
            new_state.pending_cycle = 0;
            new_state
        }
        "addx" => {
            new_state.pending_command = cmd;
            new_state.pending_cycle = 1;
            new_state
        }
        e => unreachable!("unknown command: {}", e),
    }
}

fn calculate_pending_command(state: &State) -> isize {
    let mut new_x_register = state.x_register;
    match state.pending_command {
        Command { op: "noop", .. } => {}
        Command {
            op: "addx",
            val: Some(v),
        } => {
            new_x_register += v;
        }
        c => unreachable!("unknown command: {c:?}"),
    }
    new_x_register
}
