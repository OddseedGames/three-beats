pub mod action;
mod resolve;
pub mod state;

pub use action::{Action, Beat};
pub use resolve::resolve;
pub use state::{Duel, Fighter, Moment};

pub const CELLS: i8 = 7;
pub const MAX_ENERGY: i16 = 8;
