use crate::GameState;
use crate::loading::TextureAssets;
use crate::util::despawn_all;

use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Splash)
                    .with_system(setup_splashscreen)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Splash)
                    .with_system(update_splashscreen)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Splash)
                    .with_system(despawn_all::<OnSplashScreen>)
            );
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Deref, DerefMut)]
struct SplashScreenTimer(Timer);

fn setup_splashscreen(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    let icon = textures.bevy.clone();    

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Px(200.), Val::Auto),
                ..Default::default()
            },
            image: UiImage(icon),
            ..Default::default()
        })
        .insert(OnSplashScreen);
    
    commands
        .insert_resource(SplashScreenTimer(Timer::from_seconds(2.5, false)));

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(OnSplashScreen);
}

fn update_splashscreen(
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashScreenTimer>,
    mut commands: Commands,
) {
    if timer.tick(time.delta()).finished() {
        state.set(GameState::MainMenu).unwrap();
        commands.remove_resource::<SplashScreenTimer>();
    }
}