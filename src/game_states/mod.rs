use bevy::prelude::*;

mod in_game;
mod main_menu;
mod paused;
use in_game::{in_game, in_game_exit, setup_in_game};
use main_menu::{main_menu, main_menu_exit, setup_main_menu};
use paused::{paused, paused_exit, setup_paused};

/// Game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            // MainMenu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, main_menu.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), main_menu_exit)
            // InGame
            .add_systems(OnEnter(GameState::InGame), setup_in_game)
            .add_systems(Update, in_game.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), in_game_exit)
            // Paused
            .add_systems(OnEnter(GameState::Paused), setup_paused)
            .add_systems(Update, paused.run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), paused_exit);
    }
}
