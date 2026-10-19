use macroquad::prelude::*;

pub const W: f32 = 1280.0;
pub const H: f32 = 920.0;
pub const HELP: Rect = Rect::new(1008.0, 33.0, 52.0, 36.0);
pub const SOUND: Rect = Rect::new(1070.0, 33.0, 92.0, 36.0);
pub const RESTART: Rect = Rect::new(1172.0, 33.0, 80.0, 36.0);
pub const COMMIT: Rect = Rect::new(1074.0, 543.0, 178.0, 116.0);
pub const OVERLAY_BUTTON: Rect = Rect::new(437.0, 574.0, 406.0, 52.0);
pub const UNDO: Rect = Rect::new(28.0, 646.0, 102.0, 26.0);
pub const CLEAR: Rect = Rect::new(140.0, 646.0, 112.0, 26.0);

pub fn card(index: usize) -> Rect {
    Rect::new(
        28.0 + (index % 4) as f32 * 309.0,
        710.0 + (index / 4) as f32 * 82.0,
        297.0,
        72.0,
    )
}
pub fn slot(index: usize, enemy: bool) -> Rect {
    Rect::new(
        280.0 + index as f32 * 258.0,
        if enemy { 543.0 } else { 607.0 },
        244.0,
        52.0,
    )
}
pub fn fighter_x(cell: f32) -> f32 {
    214.0 + cell * 142.0
}
pub fn pointer() -> Vec2 {
    let scale = (screen_width() / W).min(screen_height() / H);
    let offset = vec2(
        (screen_width() - W * scale) / 2.0,
        (screen_height() - H * scale) / 2.0,
    );
    (Vec2::from(mouse_position()) - offset) / scale
}
pub fn camera() -> Camera2D {
    let scale = (screen_width() / W).min(screen_height() / H);
    Camera2D {
        target: vec2(W / 2.0, H / 2.0),
        zoom: vec2(2.0 * scale / screen_width(), 2.0 * scale / screen_height()),
        ..Default::default()
    }
}
