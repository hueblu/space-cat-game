use crate::GameState;
use crate::loading::TextureAssets;
use crate::util::despawn_all;

use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Splash)
                    .with_system(setup_splashscreen)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Splash)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Splash)
                    .with_system(despawn_all::<OnSplashScreen>)
            );
    }
}

#[derive(Component)]
struct OnSplashScreen;

fn setup_splashscreen(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.bevy.clone(),
            ..Default::default()
        })
        .insert(OnSplashScreen);
    
    commands
        .init_resource();
}

