use crate::combat::{Action, Beat, MAX_ENERGY};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Plan {
    pub actions: Vec<Action>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlanError {
    Full,
    Energy,
}

impl Plan {
    pub fn duration(&self) -> usize {
        self.actions.iter().map(|a| a.duration()).sum()
    }
    pub fn energy_after(&self, initial: i16) -> Option<i16> {
        let mut energy = initial;
        for action in &self.actions {
            if energy < action.cost() {
                return None;
            }
            energy -= action.cost();
            if *action == Action::Rest {
                energy = (energy + 3).min(MAX_ENERGY);
            }
        }
        Some(energy)
    }
    pub fn push(&mut self, action: Action, energy: i16) -> Result<(), PlanError> {
        if self.duration() + action.duration() > 3 {
            return Err(PlanError::Full);
        }
        self.actions.push(action);
        if self.energy_after(energy).is_none() {
            self.actions.pop();
            return Err(PlanError::Energy);
        }
        Ok(())
    }
    pub fn timeline(&self) -> Vec<Beat> {
        self.actions
            .iter()
            .flat_map(|action| {
                if action.duration() == 2 {
                    vec![Beat::Windup(*action), Beat::Strike(*action)]
                } else {
                    vec![Beat::Act(*action)]
                }
            })
            .collect()
    }
    pub fn code(&self) -> u32 {
        self.actions
            .iter()
            .fold(0, |code, action| code * 10 + action.key())
    }
}

pub fn legal_plans(energy: i16) -> Vec<Plan> {
    fn visit(plan: Plan, energy: i16, result: &mut Vec<Plan>) {
        if plan.duration() == 3 {
            result.push(plan);
            return;
        }
        for action in Action::ALL {
            let mut next = plan.clone();
            if next.push(action, energy).is_ok() {
                visit(next, energy, result);
            }
        }
    }
    let mut result = Vec::new();
    visit(Plan::default(), energy, &mut result);
    result
}
