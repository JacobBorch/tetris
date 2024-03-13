use bevy::ecs::query;
use bevy::input::*;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::mesh::shape::Circle;
use bevy::render::mesh::shape::Cube;
use bevy::render::mesh::shape::Quad;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::settings::RenderCreation;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2d;
use bevy::window::PresentMode;
use tetris::tetris::Piece;
use tetris::tetris::TetrisGame; // Import the PlayerInput trait
use tetris::tetris::{Game, PlayerInput}; // Import the missing Input type

struct TetrisPlugin;

#[derive(Resource)]
struct TetrisTimer(Timer);

const BLOCK_SIZE: f32 = 20.0;


impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Game::new()) // Your Tetris game instance
            .insert_resource(TetrisTimer(Timer::from_seconds(1.5, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, player_input)
            .add_systems(Update, game_tick)
            .add_systems(Update, draw_current_piece)
            .add_systems(Update, draw_board);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
}


fn game_tick(mut game: ResMut<Game>, time: Res<Time>, mut timer: ResMut<TetrisTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("tick!");
        game.tick();
    }

}

fn draw_board() {}

fn piece_pos_to_coordinates((i, j): (usize, usize)) -> Vec3 {
    vec3((10 * j) as f32, -((10 * i) as f32), 1.0)
}


#[derive(Component)]
struct PieceComponent{}

fn draw_current_piece(game: Res<Game>, old_piece: Query<Entity, With<PieceComponent>>, mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let (pos, piece) = game.get_current_piece();
    let color = piece.color();
    let vec3pos = piece_pos_to_coordinates(pos);
    old_piece.for_each(|p| commands.get_entity(p).unwrap().despawn());
    commands.spawn((MaterialMesh2dBundle {
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(Cube::new(BLOCK_SIZE).into())),
        transform: Transform::from_translation(vec3pos),
        material: materials.add(color.into()),
        ..default()
    }, PieceComponent{}));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    #[cfg(target_os = "windows")]
                    backends: Some(bevy::render::settings::Backends::DX12),
                    ..Default::default()
                }),
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tetris".to_string(),
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),)
        .add_plugins(TetrisPlugin)
        .run();
}

fn player_input(keyboard_input: Res<Input<KeyCode>>, mut game: ResMut<Game>) {
    if keyboard_input.pressed(KeyCode::Left) {
        game.player_input(PlayerInput::Left);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        game.player_input(PlayerInput::Right);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        game.player_input(PlayerInput::Rotate);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        game.player_input(PlayerInput::Place);
    }
    if keyboard_input.pressed(KeyCode::Z) {
        game.player_input(PlayerInput::Swap);
    }
}

