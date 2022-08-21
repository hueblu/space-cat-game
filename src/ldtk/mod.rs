use crate::GameState;

mod components;

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, LdtkPlugin as EcsLdtkPlugin};





pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EcsLdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup)
            );
    }
}

fn setup(
    mut commands: Commands
) {
    commands
        .spawn_bundle(Camera2dBundle::default());
}
