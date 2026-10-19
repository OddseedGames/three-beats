use super::theme::*;
use crate::{app::Session, layout::*};
use macroquad::prelude::*;
use oddseed_three_beats::{ai, combat::MAX_ENERGY};

pub fn draw(session: &Session, font: &Font, point: Vec2) {
    let game = &session.game;
    text(font, "ODDSEED / 003", 28.0, 30.0, 11.0, AMBER);
    text(font, "ТРИ ТАКТА", 26.0, 76.0, 40.0, INK);
    centered(
        font,
        &format!("БОЙ {} / 3", game.stage + 1),
        650.0,
        33.0,
        12.0,
        MUTED,
    );
    centered(
        font,
        &format!("РАУНД {:02}", game.round),
        650.0,
        72.0,
        22.0,
        INK,
    );
    for (rect, label) in [
        (HELP, "?"),
        (
            SOUND,
            if session.muted {
                "ЗВУК -"
            } else {
                "ЗВУК +"
            },
        ),
        (RESTART, "ЗАНОВО"),
    ] {
        panel(rect, if rect.contains(point) { LINE } else { PANEL }, LINE);
        centered(
            font,
            label,
            rect.x + rect.w / 2.0,
            rect.y + 24.0,
            12.0,
            MUTED,
        );
    }
    for side in 0..2 {
        let x = if side == 0 { 28.0 } else { 762.0 };
        let fighter = game.duel.fighters[side];
        let color = if side == 0 { AMBER } else { TEAL };
        let name = if side == 0 {
            "НОВА / ТВОЙ БОЕЦ"
        } else {
            ai::NAMES[game.stage]
        };
        text(font, name, x, 117.0, 15.0, color);
        right(
            font,
            &format!("{} / {}", fighter.health, fighter.max_health),
            x + 490.0,
            117.0,
            15.0,
            INK,
        );
        draw_rectangle(x, 129.0, 490.0, 8.0, LINE);
        draw_rectangle(
            x,
            129.0,
            490.0 * fighter.health as f32 / fighter.max_health as f32,
            8.0,
            color,
        );
        text(font, "ВЫНОСЛИВОСТЬ", x, 163.0, 10.0, MUTED);
        for i in 0..MAX_ENERGY {
            draw_rectangle(
                x + 145.0 + i as f32 * 18.0,
                151.0,
                12.0,
                13.0,
                if i < fighter.energy { color } else { LINE },
            );
        }
        right(
            font,
            if side == 0 {
                "ДУМАЙ. БЕЙ."
            } else {
                ai::STYLES[game.stage]
            },
            x + 490.0,
            181.0,
            10.0,
            MUTED,
        );
    }
    centered(font, "VS", 640.0, 145.0, 22.0, MUTED);
    draw_line(28.0, 885.0, 1252.0, 885.0, 1.0, LINE);
    text(
        font,
        "1-8  приёмы    BACKSPACE  отменить    C  очистить    ENTER  бой",
        28.0,
        909.0,
        11.0,
        MUTED,
    );
    right(
        font,
        "H  правила    P  пауза    R  заново",
        1252.0,
        909.0,
        11.0,
        MUTED,
    );
}
