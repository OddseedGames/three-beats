use crate::app::Session;
use std::sync::atomic::{AtomicBool, Ordering};
static PAUSE: AtomicBool = AtomicBool::new(false);

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn three_beats_ready();
    fn three_beats_status(
        phase: u32,
        stage: u32,
        round: u32,
        beat: u32,
        player_health: i32,
        enemy_health: i32,
        player_energy: i32,
        enemy_energy: i32,
        player_cell: i32,
        enemy_cell: i32,
        slots: u32,
        queued: u32,
        enemy_plan: u32,
        paused: u32,
        help: u32,
    );
}

#[no_mangle]
pub extern "C" fn three_beats_pause() {
    PAUSE.store(true, Ordering::Relaxed);
}
pub fn pause_requested() -> bool {
    PAUSE.swap(false, Ordering::Relaxed)
}
pub fn ready() {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        three_beats_ready();
    }
}
pub fn report(_session: &Session) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        let game = &_session.game;
        let [player, enemy] = game.duel.fighters;
        three_beats_status(
            game.phase as u32,
            game.stage as u32,
            game.round,
            game.beat as u32,
            player.health as i32,
            enemy.health as i32,
            player.energy as i32,
            enemy.energy as i32,
            player.cell as i32,
            enemy.cell as i32,
            game.player_plan.duration() as u32,
            game.player_plan.code(),
            game.enemy_plan.code(),
            _session.paused as u32,
            _session.help as u32,
        );
    }
}
