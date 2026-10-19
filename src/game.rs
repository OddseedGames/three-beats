use crate::{
    ai,
    combat::{resolve, Duel, Moment, MAX_ENERGY},
    plan::Plan,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Intro,
    Planning,
    Playback,
    WonBout,
    Lost,
    Champion,
}

pub struct Game {
    pub phase: Phase,
    pub stage: usize,
    pub round: u32,
    pub duel: Duel,
    pub player_plan: Plan,
    pub enemy_plan: Plan,
    pub replay: Vec<Moment>,
    pub beat: usize,
    pub summary: String,
}

impl Default for Game {
    fn default() -> Self {
        let duel = Duel::new(ai::HEALTH[0]);
        Self {
            phase: Phase::Intro,
            stage: 0,
            round: 1,
            enemy_plan: ai::choose(0, 1, &duel),
            duel,
            player_plan: Plan::default(),
            replay: Vec::new(),
            beat: 0,
            summary: "Прочитай намерение. Собери три такта. Нажми БОЙ.".to_owned(),
        }
    }
}

impl Game {
    pub fn start(&mut self) {
        *self = Self::default();
        self.phase = Phase::Planning;
    }
    pub fn launch(&mut self) -> bool {
        if self.phase != Phase::Planning || self.player_plan.duration() != 3 {
            return false;
        }
        let player = self.player_plan.timeline();
        let enemy = self.enemy_plan.timeline();
        self.replay.clear();
        let mut duel = self.duel.clone();
        for i in 0..3 {
            let moment = resolve(&duel, [player[i], enemy[i]]);
            duel = moment.after.clone();
            self.replay.push(moment);
            if duel.over() {
                break;
            }
        }
        self.phase = Phase::Playback;
        self.beat = 0;
        self.duel = self.replay[0].after.clone();
        true
    }
    pub fn advance(&mut self) {
        if self.phase != Phase::Playback {
            return;
        }
        if self.beat + 1 < self.replay.len() {
            self.beat += 1;
            self.duel = self.replay[self.beat].after.clone();
            return;
        }
        self.summary = self
            .replay
            .iter()
            .flat_map(|moment| moment.messages.iter())
            .cloned()
            .collect::<Vec<_>>()
            .join(" · ");
        if self.duel.fighters[0].health == 0 {
            self.phase = Phase::Lost;
        } else if self.duel.fighters[1].health == 0 {
            self.phase = if self.stage == 2 {
                Phase::Champion
            } else {
                Phase::WonBout
            };
        } else {
            self.round += 1;
            for fighter in &mut self.duel.fighters {
                fighter.energy = (fighter.energy + 1).min(MAX_ENERGY);
                fighter.pending = None;
            }
            self.player_plan = Plan::default();
            self.enemy_plan = ai::choose(self.stage, self.round, &self.duel);
            self.phase = Phase::Planning;
        }
    }
    pub fn next_bout(&mut self) {
        if self.phase != Phase::WonBout {
            return;
        }
        self.stage += 1;
        self.round = 1;
        self.duel = Duel::new(ai::HEALTH[self.stage]);
        self.player_plan = Plan::default();
        self.enemy_plan = ai::choose(self.stage, self.round, &self.duel);
        self.replay.clear();
        self.phase = Phase::Planning;
        self.summary = ai::STYLES[self.stage].to_owned();
    }
}
