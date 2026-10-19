use crate::app::Session;
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};
use oddseed_three_beats::game::Phase;

pub struct Audio {
    hit: Sound,
    bell: Sound,
}
impl Audio {
    pub async fn load() -> Self {
        Self {
            hit: load_sound_from_bytes(include_bytes!("../assets/hit.wav"))
                .await
                .unwrap(),
            bell: load_sound_from_bytes(include_bytes!("../assets/bell.wav"))
                .await
                .unwrap(),
        }
    }
    pub fn update(&self, session: &Session, before: (Phase, usize)) {
        if session.muted {
            return;
        }
        let game = &session.game;
        if game.phase == Phase::Playback && (before.0 != game.phase || before.1 != game.beat) {
            let moment = &game.replay[game.beat];
            play_sound(
                if moment.damage.iter().any(|d| *d > 0) {
                    &self.hit
                } else {
                    &self.bell
                },
                PlaySoundParams {
                    looped: false,
                    volume: 0.3,
                },
            );
        }
    }
}
