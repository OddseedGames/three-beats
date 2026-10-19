use super::{Action, Beat, Duel, Moment, CELLS, MAX_ENERGY};

pub fn resolve(duel: &Duel, requested: [Beat; 2]) -> Moment {
    let mut after = duel.clone();
    let mut beats = requested;
    let mut messages = Vec::new();
    let names = ["НОВА", "СОПЕРНИК"];
    for side in 0..2 {
        let fighter = &mut after.fighters[side];
        if let Beat::Strike(action) = beats[side] {
            if fighter.pending != Some(action) {
                beats[side] = Beat::Canceled(action);
                messages.push(format!("{}: атака сорвана", names[side]));
            }
            fighter.pending = None;
        } else {
            let action = beats[side].action();
            if fighter.energy < action.cost() {
                beats[side] = Beat::Act(Action::Rest);
                fighter.pending = None;
            } else {
                fighter.energy -= action.cost();
                fighter.pending = if matches!(beats[side], Beat::Windup(_)) {
                    Some(action)
                } else {
                    None
                };
            }
            if beats[side] == Beat::Act(Action::Rest) {
                fighter.energy = (fighter.energy + 3).min(MAX_ENERGY);
            }
        }
    }
    let mut destinations = [after.fighters[0].cell, after.fighters[1].cell];
    for side in 0..2 {
        let toward = if side == 0 { 1 } else { -1 };
        let delta = match beats[side] {
            Beat::Act(Action::Advance) => toward,
            Beat::Act(Action::Retreat) => -toward,
            _ => 0,
        };
        destinations[side] = (destinations[side] + delta).clamp(0, CELLS - 1);
    }
    if destinations[0] < destinations[1] {
        for (side, cell) in destinations.into_iter().enumerate() {
            after.fighters[side].cell = cell;
        }
    }
    let mut damage = [0; 2];
    let mut push = [false; 2];
    for side in 0..2 {
        let action = beats[side].action();
        if matches!(beats[side], Beat::Windup(_) | Beat::Canceled(_)) || action.damage() == 0 {
            continue;
        }
        let other = 1 - side;
        if after.distance() > action.range() {
            messages.push(format!("{}: мимо", names[side]));
        } else if beats[other] == Beat::Act(Action::Dodge) {
            messages.push(format!("{}: уклон", names[other]));
        } else {
            let blocked = beats[other] == Beat::Act(Action::Block);
            damage[other] = if blocked {
                (action.damage() + 2) / 3
            } else {
                action.damage()
            };
            push[other] = !blocked && matches!(action, Action::Heavy | Action::Kick);
            messages.push(format!(
                "{}: -{}{}",
                names[other],
                damage[other],
                if blocked { " через блок" } else { "" }
            ));
        }
    }
    let mut interrupted = [false; 2];
    for side in 0..2 {
        after.fighters[side].health = (after.fighters[side].health - damage[side]).max(0);
        if damage[side] > 0 && after.fighters[side].pending.is_some() {
            after.fighters[side].pending = None;
            interrupted[side] = true;
            messages.push(format!("{}: замах прерван", names[side]));
        }
        if push[side] {
            let away = if side == 0 { -1 } else { 1 };
            after.fighters[side].cell = (after.fighters[side].cell + away).clamp(0, CELLS - 1);
        }
    }
    Moment {
        before: duel.clone(),
        after,
        beats,
        damage,
        interrupted,
        messages,
    }
}
