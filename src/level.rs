use bevy::prelude::*;



pub struct LevelPlugin; 

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(PlayState::Disabled);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum PlayState {
    Disabled
}