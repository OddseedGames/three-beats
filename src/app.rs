use crate::{
    audio::Audio,
    browser,
    input::{self, Command},
    layout::*,
    view,
};
use macroquad::prelude::*;
use oddseed_three_beats::{
    game::{Game, Phase},
    plan::PlanError,
};

pub const BEAT_SECONDS: f32 = 0.85;

pub struct Session {
    pub game: Game,
    pub paused: bool,
    pub help: bool,
    pub muted: bool,
    pub clock: f32,
    pub notice: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            game: Game::default(),
            paused: false,
            help: false,
            muted: true,
            clock: 0.0,
            notice: String::new(),
        }
    }
}

impl Session {
    fn command(&mut self, command: Command) {
        match command {
            Command::Sound => self.muted = !self.muted,
            Command::Help => self.help = !self.help,
            Command::Pause => {
                if self.help {
                    self.help = false;
                } else {
                    self.paused = !self.paused;
                }
            }
            Command::Restart => {
                *self = Self {
                    muted: self.muted,
                    ..Self::default()
                };
                self.game.start();
            }
            Command::Commit if self.help || self.paused => {
                self.help = false;
                self.paused = false;
            }
            Command::Commit => match self.game.phase {
                Phase::Intro | Phase::Lost | Phase::Champion => self.game.start(),
                Phase::WonBout => self.game.next_bout(),
                Phase::Planning => {
                    if self.game.launch() {
                        self.clock = 0.0;
                        self.notice.clear();
                    } else {
                        self.notice = "Заполни все три такта перед боем.".to_owned();
                    }
                }
                Phase::Playback => {}
            },
            _ if self.help || self.paused || self.game.phase != Phase::Planning => {}
            Command::Add(action) => {
                self.notice = match self
                    .game
                    .player_plan
                    .push(action, self.game.duel.fighters[0].energy)
                {
                    Ok(()) => String::new(),
                    Err(PlanError::Full) => {
                        "Не хватает тактов. Backspace отменяет последний приём.".to_owned()
                    }
                    Err(PlanError::Energy) => {
                        "Не хватает выносливости. Добавь передышку клавишей 8.".to_owned()
                    }
                };
            }
            Command::Undo => {
                self.game.player_plan.actions.pop();
                self.notice.clear();
            }
            Command::Clear => {
                self.game.player_plan.actions.clear();
                self.notice.clear();
            }
            Command::None => {}
        }
    }
}

pub fn config() -> Conf {
    Conf {
        window_title: "Три такта · OddseedGames".to_owned(),
        window_width: W as i32,
        window_height: H as i32,
        high_dpi: true,
        ..Default::default()
    }
}

pub async fn run() {
    let font = view::theme::load_font();
    let audio = Audio::load().await;
    let mut session = Session::default();
    browser::ready();
    loop {
        if browser::pause_requested() && session.game.phase == Phase::Playback {
            session.paused = true;
        }
        let point = pointer();
        let previous = (session.game.phase, session.game.beat);
        session.command(input::poll(&session, point));
        if session.game.phase == Phase::Playback && !session.paused && !session.help {
            session.clock += get_frame_time().min(0.05);
            if session.clock >= BEAT_SECONDS {
                session.clock = 0.0;
                session.game.advance();
            }
        }
        audio.update(&session, previous);
        view::draw(&session, &font, point);
        browser::report(&session);
        next_frame().await;
    }
}
