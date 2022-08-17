mod loading;
mod splashscreen;
mod main_menu;
mod actions;
mod level;
mod util;

use loading::LoadingPlugin;
use splashscreen::SplashPlugin;
use main_menu::MainMenuPlugin;
use actions::ActionsPlugin;
use level::LevelPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum GameState {
    AssetLoading,
    Splash,
    MainMenu,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ActionsPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(SplashPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(LevelPlugin)
            .add_state(GameState::AssetLoading);

        #[cfg(debug_assertions)]
        {
            app
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default());
        }
    }
}

