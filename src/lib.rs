mod assets;
mod splashscreen;
mod main_menu;
mod util;

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
            .add_plugin(assets::LoadingPlugin)
            .add_plugin(splashscreen::SplashPlugin)
            .add_plugin(main_menu::MainMenuPlugin)
            .add_state(GameState::AssetLoading);

        #[cfg(debug_assertions)]
        {
            app
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default());
        }
    }
}

