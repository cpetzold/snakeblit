mod board;

use bevy::{
    math::{uvec2, vec2},
    prelude::*,
    render::camera::ScalingMode,
    sprite::Anchor,
    window::WindowMode,
};
use board::Board;

const BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

const CELL_SIZE: usize = 40;
const WINDOW_WIDTH: usize = 1280;
const WINDOW_HEIGHT: usize = 720;
const BOARD_WIDTH: usize = WINDOW_WIDTH / CELL_SIZE;
const BOARD_HEIGHT: usize = WINDOW_HEIGHT / CELL_SIZE;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "snakeblit".to_string(),
            // width: (BOARD_WIDTH * CELL_SIZE) as f32,
            // height: (BOARD_HEIGHT * CELL_SIZE) as f32,
            // mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(Board::new(BOARD_HEIGHT, BOARD_WIDTH))
        .add_startup_system(setup)
        .add_system(snake_movement)
        .add_system(sync_transforms_with_board)
        .run();
}

fn grid_to_world(pos: IVec2) -> Vec2 {
    (pos * CELL_SIZE as i32).as_vec2()
}

#[derive(Component)]
struct SnakeHead;

#[derive(Clone, Copy)]
enum SnakeSegmentColor {
    Blue,
    Red,
}

impl SnakeSegmentColor {
    fn color(&self) -> Color {
        match self {
            SnakeSegmentColor::Blue => Color::ALICE_BLUE,
            SnakeSegmentColor::Red => Color::RED,
        }
    }
}

#[derive(Component)]
struct SnakeSegment {
    color: SnakeSegmentColor,
}

#[derive(Bundle)]
struct SnakeSegmentBundle {
    snake_segment: SnakeSegment,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl SnakeSegmentBundle {
    fn new(segment_color: SnakeSegmentColor) -> Self {
        Self {
            snake_segment: SnakeSegment {
                color: segment_color,
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    color: segment_color.color(),
                    custom_size: Some(vec2(CELL_SIZE as f32, CELL_SIZE as f32)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands, mut board: ResMut<Board>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH as f32 / 2., WINDOW_HEIGHT as f32 / 2., 0.),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Auto {
                min_width: WINDOW_WIDTH as f32,
                min_height: WINDOW_HEIGHT as f32,
            },
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            color: BACKGROUND_COLOR,
            custom_size: Some(vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
            ..default()
        },
        ..default()
    });

    let head_position = uvec2(BOARD_WIDTH as u32 / 2, BOARD_HEIGHT as u32 / 2);
    let head = commands
        .spawn_bundle(SnakeSegmentBundle::new(SnakeSegmentColor::Blue))
        .insert(SnakeHead)
        .id();

    board.set(head_position, head).unwrap();

    let segment = commands
        .spawn_bundle(SnakeSegmentBundle::new(SnakeSegmentColor::Red))
        .id();

    board.set(uvec2(1, 4), segment).unwrap();
}

fn snake_movement(
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, With<SnakeHead>>,
    mut board: ResMut<Board>,
) {
    let head = query.single();

    let offset = if keys.just_pressed(KeyCode::Left) {
        IVec2::NEG_X
    } else if keys.just_pressed(KeyCode::Right) {
        IVec2::X
    } else if keys.just_pressed(KeyCode::Up) {
        IVec2::Y
    } else if keys.just_pressed(KeyCode::Down) {
        IVec2::NEG_Y
    } else {
        return;
    };

    if let Err(error) = board.move_item(head, offset) {
        println!("{}", error);
    }
}

fn sync_transforms_with_board(mut query: Query<(Entity, &mut Transform)>, board: Res<Board>) {
    for (entity, mut transform) in query.iter_mut() {
        if let Some(pos) = board.positions.get(&entity) {
            transform.translation = grid_to_world(pos.as_ivec2()).extend(transform.translation.z);
        }
    }
}
