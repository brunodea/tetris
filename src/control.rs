use bevy::prelude::*;

use crate::{block, game, grid, piece};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, (rotate_shape_t_piece, toggle_game_state));
    }
}

// FIXME: so dumb I have to create a function per shape.
fn rotate_shape_t_piece(
    mut query_piece: Query<
        (&mut piece::CurrentDisposition, &grid::GridPosition),
        With<piece::model::ShapeT>,
    >,
    mut query_block: Query<&mut Transform, With<piece::ActivePiece>>,
    grid: Single<&grid::Grid>,
    block_size: Res<block::BlockSize>,
    shape_t_data: Res<piece::model::ShapeTData>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (mut current_disposition, grid_position) in query_piece.iter_mut() {
            for mut transform in query_block.iter_mut() {
                piece::rotate_standard_piece(
                    &shape_t_data.0.dispositions,
                    &mut current_disposition,
                    &grid_position,
                    &mut transform,
                    &grid.position,
                    *block_size,
                );
            }
        }
    }
}

fn toggle_game_state(mut game_state: ResMut<game::GameState>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        if game_state.0 == game::State::Running {
            *game_state = game::GameState(game::State::Paused);
            info!("Game is paused!");
        } else if game_state.0 == game::State::Paused {
            *game_state = game::GameState(game::State::Running);
            info!("Game is running!");
        }
    }
}
