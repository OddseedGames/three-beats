use super::theme::*;
use crate::{app::Session, layout::*};
use macroquad::prelude::*;
use oddseed_three_beats::{ai, game::Phase};

pub fn draw(session: &Session, font: &Font, point: Vec2) {
    let phase = session.game.phase;
    if !session.help && !session.paused && matches!(phase, Phase::Planning | Phase::Playback) {
        return;
    }
    draw_rectangle(0.0, 94.0, W, H - 94.0, alpha(BG, 0.92));
    panel(Rect::new(270.0, 208.0, 740.0, 474.0), rgb(18, 29, 35), LINE);
    let (eyebrow, title, subtitle, button_label) = if session.help {
        (
            "ПРАВИЛА БОЯ",
            "ДУМАЙ НА ТРИ ВПЕРЁД.",
            "Дистанция, выносливость и момент удара.",
            "ПОНЯТНО / ENTER",
        )
    } else if session.paused {
        (
            "ПАУЗА",
            "ПЕРЕВЕДИ ДЫХАНИЕ.",
            "Комбинации и текущий такт сохранены.",
            "ПРОДОЛЖИТЬ / ENTER",
        )
    } else {
        match phase {
            Phase::Intro => (
                "ПОШАГОВЫЙ ФАЙТИНГ / 1 ПРОТИВ 1",
                "ТРИ ТАКТА.",
                "Реакция подождёт. Здесь решает твой план.",
                "НА РИНГ / ENTER",
            ),
            Phase::WonBout => (
                "БОЙ ВЫИГРАН",
                "ЧИСТАЯ ПОБЕДА.",
                ai::NAMES[session.game.stage + 1],
                "СЛЕДУЮЩИЙ БОЙ / ENTER",
            ),
            Phase::Lost => (
                "ТУРНИР ОКОНЧЕН",
                "НОКДАУН.",
                "Посмотри на намерение. Попробуй другую серию.",
                "ЕЩЁ ПОПЫТКА / ENTER",
            ),
            Phase::Champion => (
                "ТРИ БОЯ. ОДИН ЧЕМПИОН.",
                "ТВОЙ ТЕМП.",
                "Рывок, Контра, Гром. Ты прочитал каждого.",
                "НОВЫЙ ТУРНИР / ENTER",
            ),
            _ => unreachable!(),
        }
    };
    text(font, eyebrow, 310.0, 256.0, 11.0, AMBER);
    text(font, title, 307.0, 320.0, 38.0, INK);
    text(font, subtitle, 310.0, 360.0, 14.0, MUTED);
    let lines = if session.help || phase == Phase::Intro && !session.paused {
        [
            "1. Прочитай три такта противника сверху.",
            "2. Собери ответ клавишами 1-8 или мышью.",
            "3. Нажми БОЙ. Обе серии исполнятся вместе.",
            "Джеб срывает замах. Уклон спасает на один такт.",
            "Передышка: +3 энергии. Между сериями: +1.",
        ]
    } else if phase == Phase::WonBout && !session.paused {
        [
            "Следующий соперник уже ждёт у канатов.",
            ai::STYLES[session.game.stage + 1],
            "Перед боем здоровье и выносливость восстановятся.",
            "Сильные удары требуют замаха и двух тактов.",
            "Не забывай отходить и восстанавливать силы.",
        ]
    } else {
        [
            "Время не идёт, пока ты выбираешь действия.",
            "Передышка даёт силы для следующей атаки.",
            "Тяжёлый удар можно сорвать попаданием в замах.",
            "Бойцы не могут пройти друг сквозь друга.",
            "Точный план сильнее быстрой реакции.",
        ]
    };
    for (i, line) in lines.into_iter().enumerate() {
        text(
            font,
            line,
            310.0,
            410.0 + i as f32 * 26.0,
            13.0,
            if i < 3 { INK } else { MUTED },
        );
    }
    button(font, button_label, OVERLAY_BUTTON, point, true);
    centered(
        font,
        "ПЛАНИРУЙ БЕЗ СПЕШКИ. СЛУЧАЙНЫХ ПРОМАХОВ НЕТ.",
        640.0,
        655.0,
        10.0,
        MUTED,
    );
}
