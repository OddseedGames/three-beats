use super::theme::*;
use crate::{
    app::{Session, BEAT_SECONDS},
    layout::fighter_x,
};
use macroquad::prelude::*;
use oddseed_three_beats::{
    combat::{Action, Beat},
    game::Phase,
};

fn bone(p: Vec2, face: f32, scale: f32, a: Vec2, b: Vec2, width: f32, color: Color) {
    let a = p + vec2(a.x * face, a.y) * scale;
    let b = p + vec2(b.x * face, b.y) * scale;
    draw_line(a.x, a.y, b.x, b.y, width * scale, color);
    draw_circle(b.x, b.y, width * scale / 2.0, color);
}

pub fn draw(session: &Session, font: &Font, side: usize) {
    let game = &session.game;
    let fighter = game.duel.fighters[side];
    let active = game.phase == Phase::Playback;
    let t = if active {
        (session.clock / BEAT_SECONDS).clamp(0.0, 1.0)
    } else {
        1.0
    };
    let pulse = (t * std::f32::consts::PI).sin().max(0.0);
    let step = if active {
        game.replay[game.beat].beats[side]
    } else {
        Beat::Act(Action::Block)
    };
    let before = if active {
        game.replay[game.beat].before.fighters[side].cell
    } else {
        fighter.cell
    };
    let ease = t * t * (3.0 - 2.0 * t);
    let cell = before as f32 + (fighter.cell - before) as f32 * ease;
    let face = if side == 0 { 1.0 } else { -1.0 };
    let color = if side == 0 { AMBER } else { TEAL };
    let scale = if side == 1 && game.stage == 2 {
        1.12
    } else {
        1.0
    };
    let attack = active
        && !matches!(step, Beat::Windup(_) | Beat::Canceled(_))
        && step.action().damage() > 0;
    let dodging = active && step == Beat::Act(Action::Dodge);
    let mut p = vec2(fighter_x(cell), 447.0);
    if attack {
        p.x += face * pulse * 15.0;
    }
    if dodging {
        p += vec2(-face * 12.0, 22.0) * pulse;
    }
    draw_ellipse(p.x, 453.0, 44.0 * scale, 10.0, 0.0, alpha(BLACK, 0.45));
    let moving = before != fighter.cell;
    let stride = if moving { pulse * 15.0 } else { 0.0 };
    let pants = rgb(32, 42, 48);
    bone(
        p,
        face,
        scale,
        vec2(-9.0, -51.0),
        vec2(-18.0 - stride, -24.0),
        14.0,
        pants,
    );
    bone(
        p,
        face,
        scale,
        vec2(-18.0 - stride, -24.0),
        vec2(-25.0 - stride, 0.0),
        12.0,
        pants,
    );
    if attack && step.action() == Action::Kick {
        bone(
            p,
            face,
            scale,
            vec2(6.0, -51.0),
            vec2(28.0 + pulse * 34.0, -31.0 - pulse * 8.0),
            14.0,
            pants,
        );
        bone(
            p,
            face,
            scale,
            vec2(28.0 + pulse * 34.0, -31.0 - pulse * 8.0),
            vec2(35.0 + pulse * 60.0, -25.0 - pulse * 14.0),
            12.0,
            pants,
        );
    } else {
        bone(
            p,
            face,
            scale,
            vec2(6.0, -51.0),
            vec2(16.0 + stride, -24.0),
            14.0,
            pants,
        );
        bone(
            p,
            face,
            scale,
            vec2(16.0 + stride, -24.0),
            vec2(25.0 + stride, 0.0),
            12.0,
            pants,
        );
    }
    bone(
        p,
        face,
        scale,
        vec2(-26.0 - stride, -1.0),
        vec2(-12.0 - stride, -1.0),
        8.0,
        rgb(180, 186, 179),
    );
    bone(
        p,
        face,
        scale,
        vec2(18.0 + stride, -1.0),
        vec2(32.0 + stride, -1.0),
        8.0,
        rgb(180, 186, 179),
    );
    bone(
        p,
        face,
        scale,
        vec2(-6.0, -89.0),
        vec2(0.0, -53.0),
        32.0,
        alpha(color, 0.72),
    );
    bone(
        p,
        face,
        scale,
        vec2(-11.0, -57.0),
        vec2(11.0, -57.0),
        8.0,
        color,
    );
    let skin = rgb(190, 146, 112);
    let head = p + vec2(4.0 * face, -113.0) * scale;
    draw_ellipse(
        head.x,
        head.y,
        14.0 * scale,
        18.0 * scale,
        -face * 4.0,
        skin,
    );
    draw_circle(
        head.x - 3.0 * face * scale,
        head.y - 11.0 * scale,
        12.0 * scale,
        rgb(30, 34, 35),
    );
    draw_circle(
        head.x + 13.0 * face * scale,
        head.y + 1.0 * scale,
        4.0 * scale,
        skin,
    );
    let guarding = step == Beat::Act(Action::Block) || !active;
    let back_fist = if guarding {
        vec2(20.0, -105.0)
    } else {
        vec2(9.0, -87.0)
    };
    bone(
        p,
        face,
        scale,
        vec2(-10.0, -88.0),
        vec2(-22.0, -68.0),
        11.0,
        skin,
    );
    bone(p, face, scale, vec2(-22.0, -68.0), back_fist, 10.0, skin);
    let mut fist = if guarding {
        vec2(35.0, -99.0)
    } else {
        vec2(34.0, -88.0)
    };
    if attack {
        fist.x += pulse * 64.0;
        fist.y -= pulse * 8.0;
    }
    if matches!(step, Beat::Windup(_)) {
        fist = vec2(-22.0, -101.0);
    }
    bone(
        p,
        face,
        scale,
        vec2(10.0, -90.0),
        vec2(29.0, -66.0),
        12.0,
        skin,
    );
    bone(p, face, scale, vec2(29.0, -66.0), fist, 10.0, skin);
    for f in [back_fist, fist] {
        let q = p + vec2(f.x * face, f.y) * scale;
        draw_circle(q.x, q.y, 11.0 * scale, color);
        draw_circle(q.x + face * 2.0, q.y - 3.0, 5.0 * scale, alpha(INK, 0.25));
    }
    if active {
        let damage = game.replay[game.beat].damage[side];
        if damage > 0 {
            let hit = p + vec2(-face * 12.0, -83.0);
            draw_poly(
                hit.x,
                hit.y,
                8,
                23.0 * (1.0 - t),
                t * 45.0,
                alpha(ALERT, 1.0 - t),
            );
            centered(
                font,
                &format!("-{}", damage),
                p.x,
                p.y - 155.0 - t * 12.0,
                25.0,
                alpha(ALERT, 1.0 - t * 0.6),
            );
        }
    }
}
