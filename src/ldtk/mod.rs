mod components;

use crate::GameState;
use crate::loading::LevelAssets;
use components::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, LdtkPlugin as EcsLdtkPlugin};





pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EcsLdtkPlugin)
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup)
            );
    }
}

fn setup(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone(),
        ..Default::default()
    });
}
