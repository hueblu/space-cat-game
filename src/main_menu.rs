use crate::{
    GameState,
    loading::MenuStyles,
    util::despawn_all,
};

use bevy::{prelude::*, app::AppExit};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state(MenuState::Disabled)
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(update_menu)
                    .with_system(update_button_color)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .with_system(despawn_all::<OnMenu>)
            )
            .add_system_set(
                SystemSet::on_enter(MenuState::Main)
                    .with_system(setup_menu_main)
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Main)
                    .with_system(despawn_all::<OnMenuMain>)
            )
            .add_system_set(
                SystemSet::on_enter(MenuState::Settings)
                    .with_system(setup_menu_settings)
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Settings)
                    .with_system(despawn_all::<OnMenuSettings>)
            );
    }
}
asdfasdfd
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum MenuState {
    Disabled,
    Main,
    Settings,
}

#[derive(Component)]
struct OnMenu;

#[derive(Component)]
struct OnMenuMain;

#[derive(Component)]
struct OnMenuSettings;

#[derive(Component)]
enum ButtonAction {
    PlayGame,
    ExitGame,
    GotoMain,
    GotoSettings,
}

fn setup_menu(
    mut commands: Commands,
    mut state: ResMut<State<MenuState>>,
) {
    state.set(MenuState::Main).unwrap();

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(OnMenu);
}

fn update_menu(
    query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, button_action) in &query {
        if *interaction == Interaction::Clicked {
            match button_action {
                ButtonAction::ExitGame => app_exit_events.send(AppExit),
                ButtonAction::PlayGame => {
                    game_state.set(GameState::Playing).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                },
                ButtonAction::GotoMain => menu_state.set(MenuState::Main).unwrap(),
                ButtonAction::GotoSettings => menu_state.set(MenuState::Settings).unwrap(),
            }
        }
    }
}

fn update_button_color(
    styles: Res<MenuStyles>,
    mut query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Clicked => styles.button_color_clicked,
            Interaction::Hovered => styles.button_color_hovered,
            Interaction::None => styles.button_color,
        };
    }
}

fn setup_menu_main(
    mut commands: Commands,
    styles: Res<MenuStyles>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::PURPLE.into(),
            ..Default::default()
        })
        .insert(OnMenuMain)
        .insert(OnMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Space Cat Game",
                styles.menu_title_style.clone(),
            )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.)),
                    ..Default::default()
                })
            );

            // Play Button
            parent
                .spawn_bundle(ButtonBundle {
                    style: styles.button_style.clone(),
                    color: styles.button_color.into(),
                    ..Default::default()
                })
                .insert(ButtonAction::PlayGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Play Game",
                        styles.button_text_style.clone(),
                    ));
                });

            // Settings Button
            parent
                .spawn_bundle(ButtonBundle {
                    style: styles.button_style.clone(),
                    color: styles.button_color.into(),
                    ..Default::default()
                })
                .insert(ButtonAction::GotoSettings)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Settings",
                        styles.button_text_style.clone(),
                    ));
                });
                
            // Quit Button
            parent
                .spawn_bundle(ButtonBundle {
                    style: styles.button_style.clone(),
                    color: styles.button_color.into(),
                    ..Default::default()
                })
                .insert(ButtonAction::ExitGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Quit",
                        styles.button_text_style.clone(),
                    ));
                });
        });
}

fn setup_menu_settings(

) {

}
