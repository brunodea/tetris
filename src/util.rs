use bevy::transform::components::Transform;

use crate::{
    block::{self, SingleBlockOffset},
    grid,
};

pub fn block_transform(
    block_offset: &SingleBlockOffset,
    grid_canvas_position: &Transform,
    grid_position: &grid::GridPosition,
    block_size: &block::BlockSize,
) -> Transform {
    let grid_canvas_position = grid_canvas_position.translation;
    let x = grid_canvas_position.x
        + ((grid_position.col as i32 + block_offset.col) as f32 * block_size.0);
    let y = grid_canvas_position.y
        - ((grid_position.row as i32 + block_offset.row) as f32 * block_size.0);
    Transform::from_xyz(x, y, 0f32)
}
