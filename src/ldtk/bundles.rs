use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use std::collections::HashSet;

<<<<<<< HEAD:src/ldtk/components.rs
#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub(super) struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: Collider::cuboid(6., 14.),
                ..Default::default()
            },
            _ => ColliderBundle::default(),
        }
    }
}
=======
#[derive(Component)]
pub(super) struct OnLevel;
>>>>>>> 41d3ac582445125b472bd76962fda60b90c1a7ec:src/ldtk/bundles.rs
