use oddseed_three_beats::combat::{resolve, Action::*, Beat::*, Duel};

fn close() -> Duel {
    let mut duel = Duel::new(24);
    duel.fighters[0].cell = 2;
    duel.fighters[1].cell = 3;
    duel
}

#[test]
fn jab_hits_only_at_reach() {
    let duel = close();
    let hit = resolve(&duel, [Act(Jab), Act(Rest)]);
    assert_eq!(hit.damage, [0, 3]);
    assert_eq!(hit.after.fighters[0].energy, 6);
    let miss = resolve(&Duel::new(24), [Act(Jab), Act(Rest)]);
    assert_eq!(miss.damage, [0, 0]);
}

#[test]
fn movement_resolves_before_attacks() {
    let moment = resolve(&close(), [Act(Jab), Act(Retreat)]);
    assert_eq!(moment.after.distance(), 2);
    assert_eq!(moment.damage, [0, 0]);
}

#[test]
fn movement_cannot_cross_or_share_a_cell() {
    let moment = resolve(&close(), [Act(Advance), Act(Advance)]);
    assert_eq!(moment.after.fighters[0].cell, 2);
    assert_eq!(moment.after.fighters[1].cell, 3);
    let mut duel = close();
    duel.fighters[1].cell = 4;
    let collision = resolve(&duel, [Act(Advance), Act(Advance)]);
    assert_eq!(collision.after.fighters[0].cell, 2);
    assert_eq!(collision.after.fighters[1].cell, 4);
}

#[test]
fn retreat_stops_at_ring_boundaries() {
    let mut duel = close();
    duel.fighters[0].cell = 0;
    duel.fighters[1].cell = 6;
    assert_eq!(
        resolve(&duel, [Act(Retreat), Act(Retreat)])
            .after
            .distance(),
        6
    );
}

#[test]
fn heavy_attack_has_windup_and_pays_only_once() {
    let windup = resolve(&close(), [Windup(Heavy), Act(Rest)]);
    assert_eq!(windup.damage, [0, 0]);
    assert_eq!(windup.after.fighters[0].energy, 4);
    let hit = resolve(&windup.after, [Strike(Heavy), Act(Rest)]);
    assert_eq!(hit.damage, [0, 7]);
    assert_eq!(hit.after.fighters[0].energy, 4);
    assert_eq!(hit.after.fighters[1].cell, 4);
}

#[test]
fn a_hit_interrupts_windup_without_a_refund_or_free_block() {
    let interrupted = resolve(&close(), [Act(Jab), Windup(Heavy)]);
    assert!(interrupted.interrupted[1]);
    assert_eq!(interrupted.after.fighters[1].pending, None);
    assert_eq!(interrupted.after.fighters[1].energy, 4);
    let next = resolve(&interrupted.after, [Act(Jab), Strike(Heavy)]);
    assert_eq!(next.damage, [0, 3]);
    assert_eq!(next.beats[1], Canceled(Heavy));
}

#[test]
fn block_reduces_damage_and_prevents_knockback() {
    let windup = resolve(&close(), [Windup(Heavy), Act(Rest)]);
    let blocked = resolve(&windup.after, [Strike(Heavy), Act(Block)]);
    assert_eq!(blocked.damage, [0, 3]);
    assert_eq!(blocked.after.fighters[1].cell, 3);
    assert_eq!(blocked.after.fighters[1].energy, 7);
}

#[test]
fn dodge_avoids_only_its_own_beat_and_spends_energy() {
    let dodged = resolve(&close(), [Act(Jab), Act(Dodge)]);
    assert_eq!(dodged.damage, [0, 0]);
    assert_eq!(dodged.after.fighters[1].energy, 5);
    assert_eq!(resolve(&dodged.after, [Act(Jab), Act(Rest)]).damage, [0, 3]);
}

#[test]
fn kick_reaches_two_cells_and_pushes() {
    let mut duel = close();
    duel.fighters[1].cell = 4;
    let windup = resolve(&duel, [Windup(Kick), Act(Rest)]);
    let strike = resolve(&windup.after, [Strike(Kick), Act(Rest)]);
    assert_eq!(strike.damage, [0, 4]);
    assert_eq!(strike.after.fighters[1].cell, 5);
}

#[test]
fn attacks_on_the_same_beat_can_knock_out_both_fighters() {
    let mut duel = close();
    for fighter in &mut duel.fighters {
        fighter.health = 3;
    }
    let moment = resolve(&duel, [Act(Jab), Act(Jab)]);
    assert_eq!(moment.damage, [3, 3]);
    assert!(moment
        .after
        .fighters
        .iter()
        .all(|fighter| fighter.health == 0));
}

#[test]
fn exhausted_fighters_rest_instead_of_attacking() {
    let mut duel = close();
    duel.fighters[0].energy = 0;
    let moment = resolve(&duel, [Windup(Heavy), Act(Rest)]);
    assert_eq!(moment.after.fighters[0].energy, 3);
    assert_eq!(moment.after.fighters[0].pending, None);
    assert_eq!(moment.beats[0], Act(Rest));
}
