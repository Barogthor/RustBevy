use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy::app::AppExit;
use bevy_ecs;
use bevy_text;
use crate::{TIME_STEP, Scoreboard};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;


#[derive(Component)]
pub struct InGameEntity;

#[derive(Component)]
pub struct Paddle {
    speed: f32,
}

#[derive(Component)]
pub struct StickyPaddle;

#[derive(Component)]
pub struct BallStuck;

#[derive(Component)]
pub struct Ball {
    velocity: Vec3,
}

#[derive(Component)]
pub enum Collider {
    Solid,
    SolidLose,
    Scorable,
    Paddle,
}

pub fn quit_game(
    mut commands: Commands,
    query: Query<Entity, With<InGameEntity>>,
) {
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}


pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // ball

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(15.0, -185., 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameEntity)
        .insert(BallStuck)
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
    // paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -215.0, 0.0),
                scale: Vec3::new(120.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle { speed: 500.0 })
        .insert(Collider::Paddle)
        .insert(StickyPaddle)
        .insert(InGameEntity);

    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(InGameEntity);

    // Add walls
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameEntity)
        .insert(Collider::Solid);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameEntity)
        .insert(Collider::Solid);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameEntity)
        .insert(Collider::SolidLose);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InGameEntity)
        .insert(Collider::Solid);

    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec3::new(150.0, 30.0, 1.0);
    let bricks_width = brick_columns as f32 * (brick_size.x + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x) / 2.0, 100.0, 0.0);
    let brick_color = Color::rgb(0.5, 0.5, 1.0);
    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            // brick
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: brick_color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: brick_position,
                        scale: brick_size,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(InGameEntity)
                .insert(Collider::Scorable);
        }
    }
}

pub fn paddle_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: QuerySet<(
        QueryState<(&Ball, &mut Transform, With<BallStuck>)>,
        QueryState<(&Paddle, &mut Transform)>
    )>
    // mut query: Query<(&Paddle, &mut Transform)>,
    // mut query_ball: Query<(&Ball, &mut Transform, With<BallStuck>)>,
) {
    let mut query_paddle = query.q1();
    let (paddle, mut transform) = query_paddle.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let translation = &mut transform.translation;
    // move the paddle horizontally
    let old_x = translation.x;
    translation.x += direction * paddle.speed * TIME_STEP;
    // bound the paddle within the walls
    translation.x = translation.x.min(380.0).max(-380.0);
    let new_x = translation.x;

    let mut query_ball = query.q0();
    if !query_ball.is_empty() {
        let (_, mut ball_transform, _) = query_ball.single_mut();
        let ball_translation = &mut ball_transform.translation;
        ball_translation.x += new_x - old_x;

    }

}

pub fn throw_ball(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    query_paddle: Query<(Entity, &Paddle, With<StickyPaddle>)>,
    query_ball: Query<(Entity, &Ball, With<BallStuck>)>,
    mut commands: Commands
) {
    if query_paddle.is_empty() || query_ball.is_empty() {
        return;
    }
    let (paddle_entity, _, _) = query_paddle.single();
    let (ball_entity, _, _) = query_ball.single();

    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed && key_code == KeyCode::Space {
                commands.entity(paddle_entity).remove::<StickyPaddle>();
                commands.entity(ball_entity).remove::<BallStuck>();
            }
        }
    }
}

pub fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform, Without<BallStuck>)>) {
    if ball_query.is_empty() {
        return;
    }
    let (ball, mut transform, _) = ball_query.single_mut();
    transform.translation += ball.velocity * TIME_STEP;
}

pub fn ball_collision_system(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Ball, &Transform, Without<BallStuck>)>,
    collider_query: Query<(Entity, &Collider, &Transform )>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if ball_query.is_empty() {
        return;
    }
    let (mut ball, ball_transform, _) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();
    let velocity = &mut ball.velocity;

    // check collision with walls
    for (collider_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // scorable colliders should be despawned and increment the scoreboard on collision
            if let Collider::Scorable = *collider {
                scoreboard.score += 1;
                commands.entity(collider_entity).despawn();
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = velocity.x > 0.0,
                Collision::Right => reflect_x = velocity.x < 0.0,
                Collision::Top => reflect_y = velocity.y < 0.0,
                Collision::Bottom => reflect_y = velocity.y > 0.0,
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                velocity.x = -velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                velocity.y = -velocity.y;
            }

            // break if this collide is on a solid, otherwise continue check whether a solid is
            // also in collision
            if let Collider::Solid = *collider {
                break;
            }
            if let Collider::SolidLose = *collider {
                app_exit_events.send(AppExit);
                break;
            }
        }
    }
}
