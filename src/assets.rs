use crate::GameState;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Splash)
                    .with_collection::<TextureAssets>()
                    .with_collection::<FontAssets>()
                    .init_resource::<MenuStyles>()
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AssetLoading)
                    .with_system(spawn_camera)
            );
    }
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_bold: Handle<Font>,
}

pub struct MenuStyles {
    pub button_color: UiColor,
    pub button_color_clicked: UiColor,
    pub button_color_hovered: UiColor,
    pub button_style: Style,
    pub button_icon_style: Style,
    pub button_text_style: TextStyle,
    pub menu_title_style: TextStyle,
}

impl FromWorld for MenuStyles {
    fn from_world(world: &mut World) -> MenuStyles {
        let fonts = match world.get_resource::<FontAssets>() {
            Some(s) => s,
            None => panic!("Failed while loading styles"),
        };

        MenuStyles {
            button_color: Color::rgb(0.2, 0.2, 0.2).into(),
            button_color_clicked: Color::rgb(0.5, 0.5, 0.5).into(),
            button_color_hovered: Color::rgb(0.7, 0.7, 0.7).into(),
            button_style: Style {
                size: Size::new(Val::Px(250.), Val::Px(65.)),
                margin: UiRect::all(Val::Px(20.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            button_icon_style: Style {
                size: Size::new(Val::Px(30.), Val::Auto),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Auto,
                },
                ..Default::default()
            },
            button_text_style: TextStyle {
                font: fonts.fira_sans_bold.clone(),
                font_size: 40.,
                color: Color::rgb(0., 0., 0.).into(),
            },
            menu_title_style: TextStyle {
                font: fonts.fira_sans_bold.clone(),
                font_size: 80.,
                color: Color::rgb(0., 0., 0.).into(),
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}