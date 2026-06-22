use crate::utils::game_system_enum::GameSystem;
use std::sync::OnceLock;
use std::sync::RwLock;

static CURRENT_GAME_SYSTEM: OnceLock<RwLock<GameSystem>> = OnceLock::new();

pub fn set_game_system(gs: GameSystem) {
    CURRENT_GAME_SYSTEM
        .get_or_init(|| RwLock::new(GameSystem::default()))
        .write()
        .unwrap()
        .clone_from(&gs);
    // or: *lock = gs;
}

pub fn current_game_system() -> GameSystem {
    CURRENT_GAME_SYSTEM
        .get_or_init(|| RwLock::new(GameSystem::default()))
        .read()
        .unwrap()
        .clone()
}
