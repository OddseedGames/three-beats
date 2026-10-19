use super::{actors, theme::*};
use crate::{app::Session, layout::*};
use macroquad::prelude::*;
use oddseed_three_beats::game::Phase;

pub fn draw(session: &Session, font: &Font) {
    let r = Rect::new(28.0, 190.0, 1224.0, 300.0);
    panel(r, rgb(14, 24, 31), LINE);
    for i in 0..9 {
        draw_rectangle(
            29.0,
            191.0 + i as f32 * 25.0,
            1222.0,
            25.0,
            rgb(13 + i * 2, 22 + i * 2, 29 + i * 2),
        );
    }
    draw_triangle(
        vec2(340.0, 191.0),
        vec2(185.0, 441.0),
        vec2(545.0, 441.0),
        alpha(AMBER, 0.035),
    );
    draw_triangle(
        vec2(940.0, 191.0),
        vec2(750.0, 441.0),
        vec2(1095.0, 441.0),
        alpha(TEAL, 0.04),
    );
    for i in 0..23 {
        let x = 70.0 + i as f32 * 51.0;
        let y = 305.0 + (i as f32 * 1.7).sin() * 9.0;
        draw_circle(x, y, 8.0, rgb(22, 32, 39));
        draw_rectangle(x - 11.0, y + 7.0, 22.0, 24.0, rgb(21, 31, 38));
    }
    for y in [309.0, 352.0, 398.0] {
        draw_line(85.0, y, 1195.0, y, 2.0, alpha(MUTED, 0.25));
    }
    draw_rectangle(29.0, 440.0, 1222.0, 49.0, rgb(29, 39, 45));
    for x in [84.0, 1196.0] {
        draw_rectangle(x - 5.0, 280.0, 10.0, 168.0, rgb(47, 58, 63));
        for y in [305.0, 349.0, 394.0] {
            draw_rectangle(
                x - 10.0,
                y - 10.0,
                20.0,
                22.0,
                if x < 640.0 {
                    alpha(AMBER, 0.6)
                } else {
                    alpha(TEAL, 0.6)
                },
            );
        }
    }
    for cell in 0..7 {
        let x = fighter_x(cell as f32);
        let color = if session.game.duel.fighters[0].cell == cell {
            AMBER
        } else if session.game.duel.fighters[1].cell == cell {
            TEAL
        } else {
            LINE
        };
        draw_line(x - 50.0, 470.0, x + 50.0, 470.0, 2.0, color);
        centered(
            font,
            &format!("0{}", cell + 1),
            x,
            483.0,
            10.0,
            alpha(color, 0.75),
        );
    }
    actors::draw(session, font, 1);
    actors::draw(session, font, 0);
    text(
        font,
        "ПОДПОЛЬНЫЙ ТУРНИР / СЕМЬ ПОЗИЦИЙ",
        48.0,
        216.0,
        10.0,
        MUTED,
    );
    right(
        font,
        &format!("ДИСТАНЦИЯ  {}", session.game.duel.distance()),
        1231.0,
        216.0,
        12.0,
        INK,
    );
    if session.game.phase == Phase::Playback {
        centered(
            font,
            &format!("ТАКТ {}", session.game.beat + 1),
            640.0,
            254.0,
            20.0,
            AMBER,
        );
    } else {
        centered(font, "III", 640.0, 305.0, 62.0, alpha(INK, 0.055));
    }
}
