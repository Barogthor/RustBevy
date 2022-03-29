mod menu;
mod game;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy::app::AppExit;
use bevy_ecs;
use bevy_text;
use bevy_ecs::schedule::ShouldRun;
use crate::game::{paddle_movement_system, ball_collision_system, ball_movement_system, setup_game, quit_game, throw_ball};
use crate::menu::{setup_menu, button_system, close_menu};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
    Quit
}

pub struct Scoreboard {
    score: usize,
}

fn run_if_game(stage: Res<State<AppState>>) -> ShouldRun {
    if *stage.current() == AppState::InGame {
        ShouldRun::YesAndCheckAgain
    }
    else {
        ShouldRun::NoAndCheckAgain
    }
}

/// An implementation of the classic game "Breakout"
const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .add_state(AppState::MainMenu)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
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
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup_game)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                // .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(paddle_movement_system)
                .with_system(throw_ball)
                .with_system(ball_collision_system)
                .with_system(ball_movement_system)
                .with_system(scoreboard_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(quit_game)
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", scoreboard.score);
}