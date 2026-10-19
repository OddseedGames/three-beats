#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Jab,
    Heavy,
    Kick,
    Advance,
    Retreat,
    Block,
    Dodge,
    Rest,
}

impl Action {
    pub const ALL: [Self; 8] = [
        Self::Jab,
        Self::Heavy,
        Self::Kick,
        Self::Advance,
        Self::Retreat,
        Self::Block,
        Self::Dodge,
        Self::Rest,
    ];

    pub fn key(self) -> u32 {
        Self::ALL.iter().position(|action| *action == self).unwrap() as u32 + 1
    }
    pub fn duration(self) -> usize {
        if matches!(self, Self::Heavy | Self::Kick) {
            2
        } else {
            1
        }
    }
    pub fn cost(self) -> i16 {
        match self {
            Self::Jab => 2,
            Self::Heavy => 4,
            Self::Kick | Self::Dodge => 3,
            Self::Advance | Self::Retreat | Self::Block => 1,
            Self::Rest => 0,
        }
    }
    pub fn damage(self) -> i16 {
        match self {
            Self::Jab => 3,
            Self::Heavy => 7,
            Self::Kick => 4,
            _ => 0,
        }
    }
    pub fn range(self) -> i8 {
        if self == Self::Kick {
            2
        } else {
            1
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Jab => "ДЖЕБ",
            Self::Heavy => "ТЯЖЁЛЫЙ",
            Self::Kick => "УДАР НОГОЙ",
            Self::Advance => "ШАГ ВПЕРЁД",
            Self::Retreat => "ШАГ НАЗАД",
            Self::Block => "БЛОК",
            Self::Dodge => "УКЛОН",
            Self::Rest => "ПЕРЕДЫШКА",
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Jab => "3 урона · дистанция 1",
            Self::Heavy => "7 урона · отбрасывает",
            Self::Kick => "4 урона · дистанция 1-2",
            Self::Advance => "Приблизиться на клетку",
            Self::Retreat => "Отойти на клетку",
            Self::Block => "Оставляет треть урона",
            Self::Dodge => "Избежать удара в этот такт",
            Self::Rest => "+3 выносливости",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Beat {
    Act(Action),
    Windup(Action),
    Strike(Action),
    Canceled(Action),
}

impl Beat {
    pub fn action(self) -> Action {
        match self {
            Self::Act(a) | Self::Windup(a) | Self::Strike(a) | Self::Canceled(a) => a,
        }
    }
    pub fn label(self) -> &'static str {
        if matches!(self, Self::Canceled(_)) {
            "СОРВАНО"
        } else if matches!(self, Self::Windup(_)) {
            "ЗАМАХ"
        } else {
            self.action().name()
        }
    }
}
