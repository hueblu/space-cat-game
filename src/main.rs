use space_cat_game::GamePlugin;

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_rapier2d::prelude::*;

asdfasdf
fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Space Cat Game".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(GamePlugin)
        .run();
}