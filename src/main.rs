use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[derive(Resource)]
struct BlockSize(f32);

#[derive(Resource)]
struct InitialGridPosition(u32, u32);

#[derive(Debug, Component)]
struct GridPosition {
    col: u32,
    row: u32,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Grid {
    cols: u32,
    rows: u32,
}

#[derive(Debug)]
struct OffsetPosition(i32, i32);
impl From<(i32, i32)> for OffsetPosition {
    fn from((col, row): (i32, i32)) -> Self {
        OffsetPosition(col, row)
    }
}

struct PieceOffsets<const NumOfBlocks: usize> {
    offset_positions: [OffsetPosition; NumOfBlocks],
}

impl<const NumOfBlocks: usize> PieceOffsets<NumOfBlocks> {
    pub fn new(positions: [(i32, i32); NumOfBlocks]) -> Self {
        PieceOffsets {
            offset_positions: positions.map(OffsetPosition::from),
        }
    }
}

#[derive(Component)]
struct PieceRotations<const NumOfBlocks: usize, const NumOfRotations: usize> {
    rotations_offsets: [PieceOffsets<NumOfBlocks>; NumOfRotations],
    current: usize,
}

impl<const NumOfBlocks: usize, const NumOfRotations: usize>
    PieceRotations<NumOfBlocks, NumOfRotations>
{
    pub fn new(rotations_offsets: [PieceOffsets<NumOfBlocks>; NumOfRotations]) -> Self {
        Self {
            rotations_offsets,
            current: 0usize,
        }
    }

    pub fn cur_offsets(&self) -> &PieceOffsets<NumOfBlocks> {
        self.rotations_offsets
            .get(self.current)
            .expect("invalid value for current rotations offsets")
    }

    pub fn rotate(&mut self) {
        self.current = (self.current + 1) % NumOfRotations;
    }
}

type StandardPieceRotations = PieceRotations<4, 4>;

#[derive(Component)]
enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    fn t() -> (Shape, StandardPieceRotations) {
        (
            Shape::T,
            StandardPieceRotations::new([
                PieceOffsets::new([(0, 0), (-1, 0), (1, 0), (0, 1)]),
                PieceOffsets::new([(0, 0), (0, -1), (0, 1), (1, 0)]),
                PieceOffsets::new([(0, 0), (-1, 0), (1, 0), (0, -1)]),
                PieceOffsets::new([(0, 0), (0, -1), (0, 1), (-1, 0)]),
            ]),
        )
    }
}

#[derive(Component)]
struct ActivePiece;

fn setup(mut commands: Commands, window: Single<&Window>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2f32, window.height() / -2f32, 0.0),
    ));
}

fn spawn_grid(mut commands: Commands, block_size: Res<BlockSize>, window: Single<&Window>) {
    let block_size = block_size.0;
    let grid = Grid { cols: 10, rows: 20 };
    let grid_width = grid.cols as f32 * block_size;
    let x = (window.width() / 2f32) - (grid_width / 2f32);
    let margin_y = -block_size;

    commands.spawn((grid, Position { x, y: margin_y }, Stroke::color(WHITE)));
}

fn render_grid(
    mut commands: Commands,
    block_size: Res<BlockSize>,
    grid: Single<(&Grid, &Position, &Stroke)>,
) {
    let (grid, position, stroke) = *grid;
    let block_size = block_size.0;
    let grid_width = grid.cols as f32 * block_size;
    let grid_height = grid.rows as f32 * block_size;

    let shape = shapes::Rectangle {
        extents: Vec2::new(grid_width, grid_height),
        origin: RectangleOrigin::TopLeft,
        ..default()
    };

    let outline = ShapeBundle {
        path: GeometryBuilder::build_as(&shape),
        transform: Transform::from_xyz(position.x, position.y, 0f32),
        ..default()
    };

    commands.spawn((outline, Stroke::new(stroke.color, 2.0)));
}

fn spawn_t(mut commands: Commands, initial_grid_position: Res<InitialGridPosition>) {
    let (shape, rotations) = Shape::t();
    commands.spawn((
        shape,
        rotations,
        GridPosition {
            col: initial_grid_position.0,
            row: initial_grid_position.1,
        },
        Fill::color(RED),
        ActivePiece,
    ));
}

fn render_block(commands: &mut Commands, block_size: &BlockSize, position: Position, fill: &Fill) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(block_size.0, block_size.0),
        origin: RectangleOrigin::Center,
        ..default()
    };

    let outline = ShapeBundle {
        path: GeometryBuilder::build_as(&shape),
        transform: Transform::from_xyz(position.x, position.y, 0f32),
        ..default()
    };

    commands.spawn((outline, Fill::color(fill.color), Stroke::new(BLACK, 2.0)));
}

fn render_active_piece(
    mut commands: Commands,
    block_size: Res<BlockSize>,
    query_pieces: Query<(&StandardPieceRotations, &GridPosition, &Fill), With<ActivePiece>>,
    grid_canvas_position: Single<&Position, With<Grid>>,
) {
    let initial_x = grid_canvas_position.x;
    let initial_y = grid_canvas_position.y;

    for (rotations, piece_grid_position, color) in &query_pieces {
        for block_offset in rotations.cur_offsets().offset_positions.as_ref() {
            let x = initial_x
                + ((piece_grid_position.col as i32 + block_offset.0) as f32 * block_size.0);
            let y = initial_y
                - ((piece_grid_position.row as i32 + block_offset.1) as f32 * block_size.0)
                - block_size.0; // minus block_size because it draws the rectangle upwards
            render_block(&mut commands, &block_size, Position { x, y }, color);
        }
    }
}

fn main() {
    App::new()
        .insert_resource(BlockSize(30f32))
        .insert_resource(InitialGridPosition(5, 0))
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(
            Startup,
            (
                setup,
                (spawn_grid, render_grid).chain(),
                (spawn_t, render_active_piece).chain(),
            ),
        )
        .run();
}
