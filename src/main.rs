mod block;
mod control;
mod debug;
mod game;
mod gravity;
mod grid;
mod piece;
mod util;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn setup(mut commands: Commands, window: Single<&Window>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2f32, window.height() / -2f32, 0.0),
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_plugins((
            block::BlockPlugin,
            control::ControlPlugin,
            debug::DebugPlugin,
            game::GamePlugin,
            gravity::GravityPlugin,
            grid::GridPlugin,
            piece::PiecePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}
