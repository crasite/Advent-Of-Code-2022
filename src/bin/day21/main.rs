use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

struct Dice {
    current: u32,
    max: u32,
    total_role: u32,
}

impl Dice {
    fn new() -> Self {
        Dice {
            current: 1,
            max: 100,
            total_role: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        let tmp = self.current;
        self.current += 1;
        if self.current > self.max {
            self.current = 1;
        }
        self.total_role += 1;
        tmp
    }
}

#[derive(Clone, Copy)]
struct Player {
    current_location: usize,
    current_score: usize,
    max_location: usize,
}

impl Player {
    fn new(starting_location: usize) -> Self {
        Player {
            current_location: starting_location,
            current_score: 0,
            max_location: 10,
        }
    }

    fn move_forward(&mut self, count: usize) {
        self.current_location += count % self.max_location;
        if self.current_location > self.max_location {
            self.current_location %= self.max_location;
        }
        self.current_score += self.current_location;
    }
}

type Cache = Arc<
    parking_lot::lock_api::RwLock<
        parking_lot::RawRwLock,
        HashMap<(usize, usize, usize, usize, bool), (usize, usize)>,
    >,
>;

fn main() {
    part_1();
    let cache: Cache = Arc::new(RwLock::new(HashMap::<
        (usize, usize, usize, usize, bool),
        (usize, usize),
    >::new()));
    let p1 = Player::new(7);
    let p2 = Player::new(8);
    let rs = calculate_reality(p1, p2, true, cache.clone());
    dbg!(rs.0.max(rs.1));
}

// return (p1win,p2win)
fn calculate_reality(p1: Player, p2: Player, is_p1: bool, cache: Cache) -> (usize, usize) {
    if let Some(v) = cache.read().get(&(
        p1.current_score,
        p2.current_score,
        p1.current_location,
        p2.current_location,
        is_p1,
    )) {
        return v.clone();
    }
    let mut p1win = 0;
    let mut p2win = 0;
    let p = if is_p1 { p1 } else { p2 };
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        1,
        3,
    );
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        3,
        4,
    );
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        6,
        5,
    );
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        7,
        6,
    );
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        6,
        7,
    );
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        3,
        8,
    );
    branching_reality(
        p.clone(),
        p1,
        p2,
        is_p1,
        &mut p1win,
        &mut p2win,
        &cache,
        1,
        9,
    );
    cache.write().insert(
        (
            p1.current_score,
            p2.current_score,
            p1.current_location,
            p2.current_location,
            is_p1,
        ),
        (p1win, p2win),
    );
    (p1win, p2win)
}

fn branching_reality(
    mut current_player: Player,
    p1: Player,
    p2: Player,
    is_p1: bool,
    p1win: &mut usize,
    p2win: &mut usize,
    cache: &Arc<
        parking_lot::lock_api::RwLock<
            parking_lot::RawRwLock,
            HashMap<(usize, usize, usize, usize, bool), (usize, usize)>,
        >,
    >,
    multiplier: usize,
    step: usize,
) {
    current_player.move_forward(step);
    if current_player.current_score >= 21 {
        if is_p1 {
            *p1win += multiplier;
        } else {
            *p2win += multiplier;
        }
    } else {
        let win = if is_p1 {
            calculate_reality(current_player, p2, false, cache.clone())
        } else {
            calculate_reality(p1, current_player, true, cache.clone())
        };
        *p1win += win.0 * multiplier;
        *p2win += win.1 * multiplier;
    }
}

fn part_1() {
    let mut dice = Dice::new();
    let mut p1 = Player::new(4);
    let mut p2 = Player::new(8);
    loop {
        let dice_value = dice.roll() + dice.roll() + dice.roll();
        p1.move_forward(dice_value as usize);
        if p1.current_score >= 1000 {
            break;
        }
        let dice_value = dice.roll() + dice.roll() + dice.roll();
        p2.move_forward(dice_value as usize);
        if p2.current_score >= 1000 {
            break;
        }
    }
    dbg!(dice.total_role as usize * usize::min(p1.current_score, p2.current_score));
}
