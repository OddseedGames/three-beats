use oddseed_three_beats::{
    ai,
    combat::{Action::*, Beat::*, Duel},
    game::{Game, Phase},
    plan::{legal_plans, Plan, PlanError},
};

#[test]
fn two_beat_actions_expand_into_windup_and_strike() {
    let mut plan = Plan::default();
    plan.push(Heavy, 8).unwrap();
    plan.push(Rest, 8).unwrap();
    assert_eq!(
        plan.timeline(),
        vec![Windup(Heavy), Strike(Heavy), Act(Rest)]
    );
    assert_eq!(plan.duration(), 3);
}

#[test]
fn full_or_unaffordable_actions_are_not_added() {
    let mut plan = Plan::default();
    assert_eq!(plan.push(Heavy, 3), Err(PlanError::Energy));
    assert!(plan.actions.is_empty());
    plan.push(Kick, 3).unwrap();
    assert_eq!(plan.push(Heavy, 3), Err(PlanError::Full));
    assert_eq!(plan.duration(), 2);
}

#[test]
fn rest_can_fund_a_later_action_without_exceeding_the_cap() {
    let mut plan = Plan::default();
    plan.push(Rest, 1).unwrap();
    plan.push(Heavy, 1).unwrap();
    assert_eq!(plan.energy_after(1), Some(0));
    let mut rest = Plan::default();
    rest.push(Rest, 8).unwrap();
    assert_eq!(rest.energy_after(8), Some(8));
}

#[test]
fn a_partial_plan_cannot_launch_and_playback_cannot_launch_again() {
    let mut game = Game::default();
    game.start();
    game.player_plan.push(Jab, 8).unwrap();
    assert!(!game.launch());
    game.player_plan.push(Heavy, 8).unwrap();
    assert!(game.launch());
    assert!(!game.launch());
}

#[test]
fn enemy_intent_is_fixed_before_player_input() {
    let mut game = Game::default();
    game.start();
    let intent = game.enemy_plan.clone();
    game.player_plan.push(Heavy, 8).unwrap();
    game.player_plan.push(Jab, 8).unwrap();
    assert_eq!(game.enemy_plan, intent);
}

#[test]
fn all_enemy_plans_are_complete_affordable_and_deterministic() {
    for stage in 0..3 {
        for round in 1..9 {
            for distance in 1..7 {
                for energy in 0..9 {
                    let mut duel = Duel::new(ai::HEALTH[stage]);
                    duel.fighters[0].cell = 0;
                    duel.fighters[1].cell = distance;
                    duel.fighters[1].energy = energy;
                    let plan = ai::choose(stage, round, &duel);
                    assert_eq!(plan.duration(), 3);
                    assert!(plan.energy_after(energy).is_some());
                    assert_eq!(plan, ai::choose(stage, round, &duel));
                }
            }
        }
    }
}

#[test]
fn finishing_a_series_restores_one_energy_and_prepares_a_new_plan() {
    let mut game = Game::default();
    game.start();
    for _ in 0..3 {
        game.player_plan.push(Block, 8).unwrap();
    }
    game.launch();
    for _ in 0..3 {
        game.advance();
    }
    assert_eq!(game.phase, Phase::Planning);
    assert_eq!(game.round, 2);
    assert_eq!(game.duel.fighters[0].energy, 6);
    assert!(game.player_plan.actions.is_empty());
    assert_eq!(game.enemy_plan.duration(), 3);
}

#[test]
fn knockout_stops_the_series_and_the_next_bout_restores_health() {
    let mut game = Game::default();
    game.start();
    game.duel.fighters[0].cell = 2;
    game.duel.fighters[1].cell = 3;
    game.duel.fighters[1].health = 3;
    game.enemy_plan = Plan::default();
    for _ in 0..3 {
        game.enemy_plan.push(Rest, 8).unwrap();
        game.player_plan.push(Jab, 8).unwrap();
    }
    game.launch();
    assert_eq!(game.replay.len(), 1);
    game.advance();
    assert_eq!(game.phase, Phase::WonBout);
    game.next_bout();
    assert_eq!(game.stage, 1);
    assert_eq!(game.duel.fighters[0].health, 24);
    assert_eq!(game.duel.fighters[0].energy, 8);
    assert_eq!(game.phase, Phase::Planning);
}

#[test]
fn restarting_clears_tournament_and_queued_actions() {
    let mut game = Game::default();
    game.start();
    game.stage = 2;
    game.player_plan.push(Heavy, 8).unwrap();
    game.start();
    assert_eq!(game.stage, 0);
    assert_eq!(game.round, 1);
    assert!(game.player_plan.actions.is_empty());
    assert!(game.replay.is_empty());
}

#[test]
fn a_tournament_can_be_won_using_only_legal_visible_plans() {
    let mut game = Game::default();
    game.start();
    for _ in 0..100 {
        if game.phase == Phase::Champion {
            return;
        }
        if game.phase == Phase::WonBout {
            game.next_bout();
        }
        assert_eq!(game.phase, Phase::Planning);
        let player = game.duel.fighters[0];
        let enemy = game.duel.fighters[1];
        let best = legal_plans(player.energy)
            .into_iter()
            .max_by(|a, b| {
                let evaluate = |plan: &Plan| {
                    let mut d = game.duel.clone();
                    let ours = plan.timeline();
                    let theirs = game.enemy_plan.timeline();
                    for i in 0..3 {
                        d = oddseed_three_beats::combat::resolve(&d, [ours[i], theirs[i]]).after;
                        if d.over() {
                            break;
                        }
                    }
                    let [p, e] = d.fighters;
                    if p.health == 0 {
                        -10000
                    } else if e.health == 0 {
                        10000 + p.health as i32
                    } else {
                        (enemy.health - e.health) as i32 * 12
                            - (player.health - p.health) as i32 * 15
                            + p.energy as i32 * 2
                            - d.distance() as i32
                    }
                };
                evaluate(a).cmp(&evaluate(b))
            })
            .unwrap();
        game.player_plan = best;
        assert!(game.launch());
        while game.phase == Phase::Playback {
            game.advance();
        }
        assert_ne!(
            game.phase,
            Phase::Lost,
            "stage {} round {}",
            game.stage,
            game.round
        );
    }
    panic!("Tournament did not finish");
}
