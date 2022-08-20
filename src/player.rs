use crate::{
    GameState,
    loading::TextureAssets,
};

use bevy::prelude::*;



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(player_setup)
            );
    }
}

fn player_setup(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.player.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });
}
