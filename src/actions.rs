use crate::GameState;

use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerActions>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_player_actions)
            );
    }
}

#[derive(Default)]
pub struct PlayerActions {
    pub movement: Vec3,
    pub menu: bool,
}

fn update_player_actions(
    mut player_actions: ResMut<PlayerActions>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    use KeyCode::*;

    let mut p_movement = Vec3::ZERO;
    if keyboard_input.any_pressed([W, Up, Space]) { p_movement.y += 1. }
    if keyboard_input.any_pressed([A, Left]) { p_movement.x -= 1. }
    if keyboard_input.any_pressed([S, Down]) { p_movement.y -= 1. }
    if keyboard_input.any_pressed([D, Right]) { p_movement.x += 1. }
    if keyboard_input.any_pressed([Escape, M]) { player_actions.menu = true }
    player_actions.movement = p_movement;
}