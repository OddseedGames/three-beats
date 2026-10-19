use crate::{app::Session, layout::*};
use macroquad::prelude::*;
use oddseed_three_beats::{combat::Action, game::Phase};

pub enum Command {
    None,
    Add(Action),
    Undo,
    Clear,
    Commit,
    Restart,
    Pause,
    Help,
    Sound,
}

pub fn poll(session: &Session, point: Vec2) -> Command {
    if is_key_pressed(KeyCode::M) {
        return Command::Sound;
    }
    if is_key_pressed(KeyCode::H) {
        return Command::Help;
    }
    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
        return Command::Pause;
    }
    if is_key_pressed(KeyCode::R) {
        return Command::Restart;
    }
    if is_key_pressed(KeyCode::Enter) {
        return Command::Commit;
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        if HELP.contains(point) {
            return Command::Help;
        }
        if SOUND.contains(point) {
            return Command::Sound;
        }
        if RESTART.contains(point) {
            return Command::Restart;
        }
        if session.help
            || session.paused
            || session.game.phase != Phase::Planning && session.game.phase != Phase::Playback
        {
            return if OVERLAY_BUTTON.contains(point) {
                Command::Commit
            } else {
                Command::None
            };
        }
        if COMMIT.contains(point) {
            return Command::Commit;
        }
        if UNDO.contains(point) {
            return Command::Undo;
        }
        if CLEAR.contains(point) {
            return Command::Clear;
        }
        for (index, action) in Action::ALL.into_iter().enumerate() {
            if card(index).contains(point) {
                return Command::Add(action);
            }
        }
    }
    if is_key_pressed(KeyCode::Backspace) {
        return Command::Undo;
    }
    if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Delete) {
        return Command::Clear;
    }
    for (index, key) in [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
    ]
    .into_iter()
    .enumerate()
    {
        if is_key_pressed(key) {
            return Command::Add(Action::ALL[index]);
        }
    }
    Command::None
}
