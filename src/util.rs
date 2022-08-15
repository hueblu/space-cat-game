use bevy::prelude::Component;

use bevy::prelude::*;

pub fn despawn_all<C: Component>(
    to_despawn: Query<Entity, With<C>>,
    mut commands: Commands,
) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}