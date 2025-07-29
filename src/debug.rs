use bevy::app::Plugin;
use bevy::{color::palettes::css::*, prelude::*};

use crate::{block, grid};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(DebugLinesEnabled(false))
            .add_systems(Update, (toggle_debug_lines_enabled, grid_debug_lines));
    }
}

#[derive(Resource)]
struct DebugLinesEnabled(bool);

fn grid_debug_lines(
    enabled: Res<DebugLinesEnabled>,
    block_size: Res<block::BlockSize>,
    grid: Single<&grid::Grid>,
    mut gizmos: Gizmos,
) {
    if enabled.0 {
        let grid = *grid;
        let position = grid.position.translation;
        let block_size = block_size.0;
        let grid_width = grid.cols as f32 * block_size;
        let grid_height = grid.rows as f32 * block_size;
        let y = -grid_height;
        for col in 1..grid.cols {
            let x = block_size * col as f32;
            gizmos.line_2d(
                Vec2::new(position.x + x, position.y),
                Vec2::new(position.x + x, position.y + y),
                BLUE_VIOLET,
            );
        }
        let x = grid_width;
        for row in 1..grid.rows {
            let y = block_size * row as f32 * -1.0;
            gizmos.line_2d(
                Vec2::new(position.x, position.y + y),
                Vec2::new(position.x + x, position.y + y),
                BLUE_VIOLET,
            );
        }
    }
}

fn toggle_debug_lines_enabled(
    mut enabled: ResMut<DebugLinesEnabled>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        enabled.0 = !enabled.0;
    }
}
