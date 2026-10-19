use super::{Action, Beat, MAX_ENERGY};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fighter {
    pub health: i16,
    pub max_health: i16,
    pub energy: i16,
    pub cell: i8,
    pub pending: Option<Action>,
}

impl Fighter {
    pub fn new(health: i16, cell: i8) -> Self {
        Self {
            health,
            max_health: health,
            energy: MAX_ENERGY,
            cell,
            pending: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duel {
    pub fighters: [Fighter; 2],
}

impl Duel {
    pub fn new(enemy_health: i16) -> Self {
        Self {
            fighters: [Fighter::new(24, 1), Fighter::new(enemy_health, 5)],
        }
    }
    pub fn over(&self) -> bool {
        self.fighters.iter().any(|fighter| fighter.health <= 0)
    }
    pub fn distance(&self) -> i8 {
        self.fighters[1].cell - self.fighters[0].cell
    }
}

#[derive(Clone, Debug)]
pub struct Moment {
    pub before: Duel,
    pub after: Duel,
    pub beats: [Beat; 2],
    pub damage: [i16; 2],
    pub interrupted: [bool; 2],
    pub messages: Vec<String>,
}
