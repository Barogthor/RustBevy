use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_ecs;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
    Quit
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
enum MenuAction {
    Play,
    Exit
}

fn quit_app(mut app_exit_events: EventWriter<AppExit> ) {
    app_exit_events.send(AppExit);
}

fn close_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}


        /// This example illustrates how to create a button that changes color and text based on its
/// interaction state.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::MainMenu)
        // .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(button_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu)
                .with_system(close_menu)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Quit)
                .with_system(quit_app)
        )
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.45, 0.45, 0.45);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &MenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>
) {
    for (interaction, mut color, action) in interaction_query.iter_mut() {
        match (*interaction, action) {
            (Interaction::Clicked, MenuAction::Play) => {
                println!("play {:?}", app_state.current());
                *color = PRESSED_BUTTON.into();
                app_state.set(AppState::InGame);
            }
            (Interaction::Hovered, MenuAction::Play) => {
                *color = HOVERED_BUTTON.into();
            }
            (Interaction::None, MenuAction::Play) => {
                *color = NORMAL_BUTTON.into();
            }
            (Interaction::Clicked, MenuAction::Exit) => {
                println!("exit {:?}", app_state.current());
                *color = PRESSED_BUTTON.into();
                app_state.set(AppState::Quit);
            }
            (Interaction::Hovered, MenuAction::Exit) => {
                *color = HOVERED_BUTTON.into();
            }
            (Interaction::None, MenuAction::Exit) => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(MenuAction::Play)
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(MenuAction::Exit)
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Exit",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}
