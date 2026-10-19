mod actors;
mod arena;
mod hud;
mod overlay;
mod planner;
pub mod theme;

use crate::{app::Session, layout::*};
use macroquad::prelude::*;

pub fn draw(session: &Session, font: &Font, point: Vec2) {
    clear_background(theme::BG);
    set_camera(&camera());
    hud::draw(session, font, point);
    arena::draw(session, font);
    planner::draw(session, font, point);
    overlay::draw(session, font, point);
    set_default_camera();
}
