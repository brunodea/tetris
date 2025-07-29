use crate::block::{self, NBlockOffsets};
use crate::{grid, util};
use bevy::color::palettes::css::{BLACK, RED};
use bevy::prelude::*;
use bevy_prototype_lyon::draw::{Fill, Stroke};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::GeometryBuilder;
use bevy_prototype_lyon::shapes::{self, RectangleOrigin};

const STANDARD_NUM_OF_BLOCKS: usize = 4;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(model::ShapeTData(model::t()))
            .add_systems(Startup, (initial_spawn_t, render_active_piece).chain());
    }
}

// FIXME: maybe should simply have a more generic spawn system for any shapes.
fn initial_spawn_t(
    commands: Commands,
    shape_t: Res<model::ShapeTData>,
    initial_grid_position: Res<grid::InitialGridPosition>,
) {
    spawn_t(
        commands,
        shape_t,
        grid::GridPosition::from(*initial_grid_position),
    );
}

fn spawn_t(
    mut commands: Commands,
    shape_t: Res<model::ShapeTData>,
    grid_position: grid::GridPosition,
) {
    info!("Spawning T at {:?}", grid_position);
    let current_disposition = CurrentDisposition(0usize);
    // FIXME: there's probably a more correct way of using Bundles.
    let piece_bundle = StandardPieceBundle {
        blocks: shape_t.0.dispositions.0[current_disposition.0],
        current_disposition,
    };
    let shape_t_bundle = model::ShapeTBundle::new(piece_bundle);
    commands.spawn((shape_t_bundle, grid_position, Fill::color(RED), ActivePiece));
}

#[derive(Component)]
pub struct ActivePiece;

pub struct PieceDispositions<const NUM_OF_BLOCKS: usize, const NUM_OF_DISPOSITIONS: usize>(
    [block::NBlockOffsets<NUM_OF_BLOCKS>; NUM_OF_DISPOSITIONS],
);

impl<const NUM_OF_BLOCKS: usize, const NUM_OF_DISPOSITIONS: usize>
    From<[[(i32, i32); NUM_OF_BLOCKS]; NUM_OF_DISPOSITIONS]>
    for PieceDispositions<NUM_OF_BLOCKS, NUM_OF_DISPOSITIONS>
{
    fn from(value: [[(i32, i32); NUM_OF_BLOCKS]; NUM_OF_DISPOSITIONS]) -> Self {
        Self(value.map(block::NBlockOffsets::from))
    }
}
impl<const NUM_OF_BLOCKS: usize, const NUM_OF_DISPOSITIONS: usize>
    PieceDispositions<NUM_OF_BLOCKS, NUM_OF_DISPOSITIONS>
{
    pub fn num_dispositions(&self) -> usize {
        NUM_OF_DISPOSITIONS
    }

    pub fn get(&self, disposition: usize) -> block::NBlockOffsets<NUM_OF_BLOCKS> {
        self.0[disposition % NUM_OF_DISPOSITIONS]
    }
}

// FIXME: perhaps the dispositions should be a config in a file read at the start of the program.

pub type StandardDispositions = PieceDispositions<STANDARD_NUM_OF_BLOCKS, 4>;

#[derive(PartialEq, Component)]
pub enum ShapeKind {
    T,
}

pub struct ShapeData<const NUM_OF_BLOCKS: usize, const NUM_OF_DISPOSITIONS: usize> {
    pub dispositions: PieceDispositions<NUM_OF_BLOCKS, NUM_OF_DISPOSITIONS>,
    pub kind: ShapeKind,
}

impl<const NUM_OF_BLOCKS: usize, const NUM_OF_DISPOSITIONS: usize>
    ShapeData<NUM_OF_BLOCKS, NUM_OF_DISPOSITIONS>
{
    pub fn num_dispositions(&mut self) -> usize {
        NUM_OF_DISPOSITIONS
    }
}

pub type StandardShapeData = ShapeData<STANDARD_NUM_OF_BLOCKS, 4>;

/// Which list of offsets is currently "enabled".
#[derive(Component)]
pub struct CurrentDisposition(usize);

// TODO: some SecretPieceBundle could be a bundle of a piece that
// holds some coins or power or something that's released after the piece is destroyed.
#[derive(Bundle)]
struct PieceBundle<const NUM_OF_BLOCKS: usize> {
    blocks: NBlockOffsets<NUM_OF_BLOCKS>,
    current_disposition: CurrentDisposition,
}

pub type StandardNBlockOffsets = NBlockOffsets<STANDARD_NUM_OF_BLOCKS>;
pub type StandardPieceBundle = PieceBundle<STANDARD_NUM_OF_BLOCKS>;

fn render_active_piece(
    mut commands: Commands,
    block_size: Res<block::BlockSize>,
    query_pieces: Query<(&StandardNBlockOffsets, &grid::GridPosition, &Fill), With<ActivePiece>>,
    grid: Single<&grid::Grid>,
) {
    for (dispositions, piece_grid_position, fill_color) in &query_pieces {
        for block_offset in dispositions.0 {
            let shape = shapes::Rectangle {
                extents: Vec2::new(block_size.0, block_size.0),
                origin: RectangleOrigin::TopLeft,
                ..default()
            };

            let outline = ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: util::block_transform(
                    &block_offset,
                    &grid.position,
                    &piece_grid_position,
                    &block_size,
                ),
                ..default()
            };

            commands.spawn((
                outline,
                Fill::color(fill_color.color),
                Stroke::new(BLACK, 1.0),
                ActivePiece,
            ));
        }
    }
}

// FIXME: move this to `util`?
pub fn rotate_standard_piece(
    dispositions: &StandardDispositions,
    current_disposition: &mut CurrentDisposition,
    grid_position: &grid::GridPosition,
    transform: &mut Transform,
    grid_canvas_position: &Transform,
    block_size: block::BlockSize,
) {
    *current_disposition = CurrentDisposition(current_disposition.0 + 1);

    // `dispositions.get()`circles between the dispositions.
    for block_offset in dispositions.get(current_disposition.0).0.iter() {
        *transform = util::block_transform(
            block_offset,
            grid_canvas_position,
            grid_position,
            &block_size,
        );
    }
}

pub mod model {
    use bevy::ecs::bundle::Bundle;
    use bevy::ecs::component::Component;
    use bevy::ecs::system::Resource;

    use crate::piece::ShapeKind;
    use crate::piece::StandardPieceBundle;
    use crate::piece::StandardShapeData;

    #[derive(Resource)]
    pub struct ShapeTData(pub StandardShapeData);

    #[derive(Component)]
    pub struct ShapeT;

    #[derive(Bundle)]
    pub struct ShapeTBundle {
        shape: ShapeT,
        pub piece: StandardPieceBundle,
    }

    impl ShapeTBundle {
        pub fn new(piece: StandardPieceBundle) -> Self {
            Self {
                shape: ShapeT,
                piece,
            }
        }
    }

    pub fn t() -> StandardShapeData {
        StandardShapeData {
            dispositions: [
                [(0, 0), (-1, 0), (1, 0), (0, 1)],
                [(0, 0), (0, -1), (0, 1), (1, 0)],
                [(0, 0), (-1, 0), (1, 0), (0, -1)],
                [(0, 0), (0, -1), (0, 1), (-1, 0)],
            ]
            .into(),
            kind: ShapeKind::T,
        }
    }
}
