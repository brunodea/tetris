use bevy::prelude::*;

use crate::{
    block, game, grid,
    piece::{self, CurrentDisposition},
    util,
};

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
    mut query_block: Query<(&mut Transform, &block::BlockIdx), With<block::ActiveBlock>>,
    grid: Single<&grid::Grid>,
    block_size: Res<block::BlockSize>,
    shape_t_data: Res<piece::model::ShapeTData>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok((mut current_disposition, piece_grid_position)) = query_piece.get_single_mut() {
            info!("Found piece to rotate.");
            *current_disposition = CurrentDisposition(
                (current_disposition.0 + 1usize) % shape_t_data.0.num_dispositions(),
            );
            let new_disposition = shape_t_data.0.dispositions.get(current_disposition.0);
            for (mut transform, idx) in query_block.iter_mut() {
                let block_offset = &new_disposition.0[idx.0];
                *transform = util::block_transform(
                    block_offset,
                    &grid.position,
                    piece_grid_position,
                    &block_size,
                );
            }
        } else {
            info!("No piece available for rotation.");
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
