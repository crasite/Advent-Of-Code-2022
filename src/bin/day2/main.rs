#[tokio::main]
async fn main() {
    let input = include_str!("input.txt");
    let mut score = 0;
    for line in input.lines() {
        let mut strat = line.split(' ');
        let elf = strat.next().unwrap();
        let me = strat.next().unwrap();
        score += scoring(elf, me);
    }
    println!("Score 1: {}", score);
    score = 0;
    for line in input.lines() {
        let mut strat = line.split(' ');
        let elf = strat.next().unwrap();
        let me = strat.next().unwrap();
        score += scoring_2(elf, me);
    }
    println!("Score 2: {}", score);
}

fn scoring(elf: &str, me: &str) -> u32 {
    match elf {
        "A" => match me {
            "X" => return 3 + 1,
            "Y" => return 6 + 2,
            "Z" => return 0 + 3,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => return 0 + 1,
            "Y" => return 3 + 2,
            "Z" => return 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => return 6 + 1,
            "Y" => return 0 + 2,
            "Z" => return 3 + 3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn scoring_2(elf: &str, me: &str) -> u32 {
    match elf {
        "A" => match me {
            "X" => return 0 + 3,
            "Y" => return 3 + 1,
            "Z" => return 6 + 2,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => return 0 + 1,
            "Y" => return 3 + 2,
            "Z" => return 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => return 0 + 2,
            "Y" => return 3 + 3,
            "Z" => return 6 + 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
