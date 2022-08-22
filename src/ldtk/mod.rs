mod bundles;

use bundles::*;
use crate::{
    GameState, 
    loading::LevelAssets,
    util::despawn_all,
};

use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, LdtkPlugin as EcsLdtkPlugin};

pub struct LdtkPlugin;

impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(EcsLdtkPlugin)
            .add_state(LevelState::Disabled)
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
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(cleanup)
                    .with_system(despawn_all::<OnLevel>)
            );
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum LevelState {
    Disabled,
    Playing,
}

pub fn setup(
    mut commands: Commands,
    mut level_state: ResMut<State<LevelState>>,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    level_state.set(LevelState::Playing).unwrap();

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone(),
        ..Default::default()
    });
}

pub fn cleanup(
    mut commands: Commands,
    mut level_state: ResMut<State<LevelState>>,
) {
    level_state.set(LevelState::Disabled).unwrap();
}