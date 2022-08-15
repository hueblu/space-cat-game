use space_cat_game::GamePlugin;

use bevy::prelude::*;



fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Space Cat Game".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
