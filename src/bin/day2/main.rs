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
            "X" => 3 + 1,
            "Y" => 6 + 2,
            "Z" => 3,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => 6 + 1,
            "Y" => 2,
            "Z" => 3 + 3,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn scoring_2(elf: &str, me: &str) -> u32 {
    match elf {
        "A" => match me {
            "X" => 3,
            "Y" => 3 + 1,
            "Z" => 6 + 2,
            _ => unreachable!(),
        },
        "B" => match me {
            "X" => 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => unreachable!(),
        },
        "C" => match me {
            "X" => 2,
            "Y" => 3 + 3,
            "Z" => 6 + 1,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
