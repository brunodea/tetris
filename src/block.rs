use bevy::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockSize(30f32));
    }
}

#[derive(Resource, Copy, Clone)]
pub struct BlockSize(pub f32);

/// Block offset relative to the center of a piece.
#[derive(Component, Debug, Copy, Clone)]
pub struct SingleBlockOffset {
    pub col: i32,
    pub row: i32,
    /// idx of the block relative to the piece it is composing.
    pub idx: usize,
}

impl From<(i32, i32, usize)> for SingleBlockOffset {
    fn from(value: (i32, i32, usize)) -> Self {
        Self {
            col: value.0,
            row: value.1,
            idx: value.2,
        }
    }
}

/// N Block offsets represent the current disposition of a piece.
#[derive(Component, Copy, Clone)]
pub struct NBlockOffsets<const NUM_OF_BLOCKS: usize>(pub [SingleBlockOffset; NUM_OF_BLOCKS]);

impl<const NUM_OF_BLOCKS: usize> From<[(i32, i32); NUM_OF_BLOCKS]>
    for NBlockOffsets<NUM_OF_BLOCKS>
{
    fn from(value: [(i32, i32); NUM_OF_BLOCKS]) -> Self {
        let mut blocks = NBlockOffsets::default();
        for (idx, _) in value.iter().enumerate() {
            blocks.0[idx] = SingleBlockOffset {
                idx,
                col: value[idx].0,
                row: value[idx].1,
            };
        }
        blocks
    }
}

impl<const NUM_OF_BLOCKS: usize> Default for NBlockOffsets<NUM_OF_BLOCKS> {
    fn default() -> Self {
        Self(
            [SingleBlockOffset {
                col: 0i32,
                row: 0i32,
                idx: 0usize,
            }; NUM_OF_BLOCKS],
        )
    }
}
