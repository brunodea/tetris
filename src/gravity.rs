use bevy::prelude::*;

use crate::{block, game, grid, piece};

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GravityTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .insert_resource(GravitySpeed(1.0))
            .add_systems(Update, (increase_gravity, apply_gravity));
    }
}

#[derive(Resource)]
struct GravityTimer(Timer);

#[derive(Resource)]
struct GravitySpeed(f32);

fn increase_gravity(
    mut timer: ResMut<GravityTimer>,
    mut speed: ResMut<GravitySpeed>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        *speed = GravitySpeed(speed.0 + 0.5);
        info!("New gravity speed: {:?}", speed.0);
        // FIXME: 2.0 should be some const variable, maybe another Res?
        *timer = GravityTimer(Timer::from_seconds(2.0 / speed.0, TimerMode::Repeating));
    }
}

fn apply_gravity(
    mut commands: Commands,
    game_state: Res<game::GameState>,
    time: Res<Time>,
    mut timer: ResMut<GravityTimer>,
    mut query_piece: Query<
        (
            Entity,
            &mut piece::StandardNBlockOffsets,
            &mut grid::GridPosition,
        ),
        With<piece::ActivePiece>,
    >,
    mut query_block: Query<&mut Transform, With<piece::ActivePiece>>,
    block_size: Res<block::BlockSize>,
    grid: Single<&grid::Grid>,
) {
    if game_state.0 == game::State::Running && timer.0.tick(time.delta()).just_finished() {
        for (entity, piece_offsets, mut grid_position) in &mut query_piece {
            grid_position.row += 1;
            for mut transform in &mut query_block {
                transform.translation.y -= block_size.0;
            }
            let mut lowest_row_offset = None;
            for offset in piece_offsets.0 {
                let block_row_offset = offset.row;
                if lowest_row_offset.is_none() {
                    lowest_row_offset = Some(block_row_offset);
                } else if let Some(lowest) = lowest_row_offset {
                    if block_row_offset > lowest {
                        lowest_row_offset = Some(block_row_offset);
                    }
                }
            }
            if let Some(lowest_row_offset) = lowest_row_offset {
                // check if the bottom block is touching the bottom of the board
                dbg!(lowest_row_offset);
                dbg!(grid_position.row);
                let piece_row = grid_position.row + lowest_row_offset as u32;
                if piece_row >= grid.rows - 1 {
                    warn!("Removed active piece!");
                    commands.entity(entity).remove::<piece::ActivePiece>();
                }
            }
        }
    }
}
