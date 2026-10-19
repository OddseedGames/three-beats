use super::theme::*;
use crate::{app::Session, layout::*};
use macroquad::prelude::*;
use oddseed_three_beats::{combat::Action, game::Phase};

pub fn draw(session: &Session, font: &Font, point: Vec2) {
    let game = &session.game;
    let notice = if !session.notice.is_empty() {
        session.notice.as_str()
    } else if game.phase == Phase::Playback {
        "Оба бойца действуют одновременно. Следи за дистанцией."
    } else if game.round > 1 && !game.summary.is_empty() {
        game.summary.as_str()
    } else {
        "Выбери три такта. Намерение противника уже зафиксировано."
    };
    let notice: String = notice.chars().take(155).collect();
    text(
        font,
        &notice,
        28.0,
        518.0,
        12.0,
        if session.notice.is_empty() {
            MUTED
        } else {
            AMBER
        },
    );
    for enemy in [true, false] {
        let plan = if enemy {
            &game.enemy_plan
        } else {
            &game.player_plan
        };
        let mut timeline = plan.timeline();
        if game.phase == Phase::Playback {
            for (i, beat) in timeline.iter_mut().enumerate().take(game.beat + 1) {
                if let Some(moment) = game.replay.get(i) {
                    *beat = moment.beats[if enemy { 1 } else { 0 }];
                }
            }
        }
        let color = if enemy { TEAL } else { AMBER };
        let y = if enemy { 563.0 } else { 627.0 };
        text(
            font,
            if enemy {
                "ПРОТИВНИК"
            } else {
                "ТВОЯ СЕРИЯ"
            },
            28.0,
            y,
            13.0,
            color,
        );
        if enemy {
            text(font, "Намерение открыто", 28.0, y + 22.0, 11.0, MUTED);
        } else {
            let energy = plan.energy_after(game.duel.fighters[0].energy).unwrap_or(0);
            text(
                font,
                &format!("{}/3 такта · остаток {}", plan.duration(), energy),
                28.0,
                y + 18.0,
                11.0,
                MUTED,
            );
        }
        for i in 0..3 {
            let r = slot(i, enemy);
            let active = game.phase == Phase::Playback && i == game.beat;
            panel(
                r,
                if active { alpha(color, 0.13) } else { PANEL },
                if active { color } else { LINE },
            );
            text(
                font,
                &format!("0{}", i + 1),
                r.x + 12.0,
                r.y + 20.0,
                10.0,
                MUTED,
            );
            centered(
                font,
                timeline.get(i).map(|b| b.label()).unwrap_or("+ ПРИЁМ"),
                r.x + r.w / 2.0 + 8.0,
                r.y + 32.0,
                13.0,
                if timeline.get(i).is_some() {
                    color
                } else {
                    MUTED
                },
            );
            if let Some(beat) = timeline.get(i) {
                if matches!(beat, oddseed_three_beats::combat::Beat::Windup(_)) {
                    draw_line(
                        r.x + 38.0,
                        r.y + 43.0,
                        r.x + r.w + 8.0,
                        r.y + 43.0,
                        2.0,
                        alpha(color, 0.5),
                    );
                }
            }
        }
    }
    for (r, label) in [(UNDO, "ОТМЕНИТЬ"), (CLEAR, "ОЧИСТИТЬ")] {
        text(
            font,
            label,
            r.x,
            r.y + 19.0,
            10.0,
            if r.contains(point) { INK } else { MUTED },
        );
    }
    let ready = game.phase == Phase::Planning && game.player_plan.duration() == 3;
    button(
        font,
        if game.phase == Phase::Playback {
            "ИДЁТ БОЙ"
        } else {
            "БОЙ"
        },
        COMMIT,
        point,
        ready,
    );
    centered(
        font,
        "ENTER",
        COMMIT.x + COMMIT.w / 2.0,
        COMMIT.y + 94.0,
        10.0,
        if ready { BG } else { MUTED },
    );
    text(font, "ПРИЁМЫ", 28.0, 695.0, 11.0, MUTED);
    right(
        font,
        "1 ТАКТ = ОДНО МГНОВЕНИЕ БОЯ",
        1252.0,
        695.0,
        10.0,
        MUTED,
    );
    for (index, action) in Action::ALL.into_iter().enumerate() {
        let r = card(index);
        let mut next = game.player_plan.clone();
        let enabled = game.phase == Phase::Planning
            && next.push(action, game.duel.fighters[0].energy).is_ok();
        let hover = r.contains(point) && enabled;
        panel(
            r,
            if hover { rgb(35, 45, 49) } else { PANEL },
            if hover { AMBER } else { LINE },
        );
        let color = if enabled { INK } else { MUTED };
        panel(Rect::new(r.x + 12.0, r.y + 12.0, 23.0, 23.0), BG, LINE);
        centered(
            font,
            &action.key().to_string(),
            r.x + 23.5,
            r.y + 29.0,
            12.0,
            AMBER,
        );
        text(font, action.name(), r.x + 46.0, r.y + 29.0, 14.0, color);
        right(
            font,
            &format!("{}Т / {}Э", action.duration(), action.cost()),
            r.x + r.w - 12.0,
            r.y + 29.0,
            10.0,
            MUTED,
        );
        text(
            font,
            action.description(),
            r.x + 14.0,
            r.y + 57.0,
            11.0,
            if hover { AMBER } else { MUTED },
        );
    }
}
