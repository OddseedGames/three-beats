use crate::{
    combat::{Action, Duel},
    plan::Plan,
};

pub const NAMES: [&str; 3] = ["РЫВОК", "КОНТРА", "ГРОМ"];
pub const STYLES: [&str; 3] = [
    "Давит сериями джебов",
    "Защищается и держит дистанцию",
    "Тяжёлые удары и отбрасывание",
];
pub const HEALTH: [i16; 3] = [16, 20, 26];

pub fn choose(stage: usize, round: u32, duel: &Duel) -> Plan {
    use Action::*;
    let distance = duel.distance();
    let desired: Vec<Action> = if distance > 2 {
        vec![Advance, Advance, if stage == 1 { Block } else { Advance }]
    } else {
        match (stage, distance, round % 3) {
            (0, 2, _) => vec![Advance, Jab, Jab],
            (0, _, 0) => vec![Heavy, Jab],
            (0, _, 1) => vec![Jab, Jab, Block],
            (0, _, _) => vec![Jab, Block, Jab],
            (1, 2, _) => vec![Kick, Block],
            (1, _, 0) => vec![Block, Jab, Retreat],
            (1, _, 1) => vec![Dodge, Jab, Rest],
            (1, _, _) => vec![Kick, Block],
            (2, 2, _) => vec![Advance, Heavy],
            (2, _, 0) => vec![Heavy, Rest],
            (2, _, 1) => vec![Jab, Heavy],
            _ => vec![Kick, Advance],
        }
    };
    let energy = duel.fighters[1].energy;
    let mut plan = Plan::default();
    for action in desired {
        if plan.duration() + action.duration() > 3 {
            break;
        }
        if plan.push(action, energy).is_err() {
            plan.push(Rest, energy).unwrap();
        }
    }
    while plan.duration() < 3 {
        plan.push(Rest, energy).unwrap();
    }
    plan
}
