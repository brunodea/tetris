use bevy::{prelude::*, reflect::GetTypeRegistration, window::PrimaryWindow};

#[derive(Resource)]
struct BlockSize(f32);

#[derive(Debug, Component)]
struct GridPosition {
    col: u32,
    row: u32,
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

enum StandardShape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Component)]
struct StandardPiece {
    shape: StandardShape,
    rotations: StandardPieceRotations,
}

#[derive(Component)]
struct Color(f32, f32, f32);

impl StandardPiece {
    fn t() -> Self {
        StandardPiece {
            shape: StandardShape::T,
            rotations: StandardPieceRotations::new([
                PieceOffsets::new([(0, 0), (-1, 0), (1, 0), (0, 1)]),
                PieceOffsets::new([(0, 0), (0, -1), (0, 1), (1, 0)]),
                PieceOffsets::new([(0, 0), (-1, 0), (1, 0), (0, -1)]),
                PieceOffsets::new([(0, 0), (0, -1), (0, 1), (-1, 0)]),
            ]),
        }
    }
}

fn setup(mut commands: Commands, window: Single<&Window>) {
    dbg!(&window.height() / -2f32);
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2f32, (window.height() / -2f32), 0.0),
    ));
}

fn add_grid(mut commands: Commands) {
    commands.spawn(Grid { cols: 10, rows: 20 });
}

fn add_initial_pieces(mut commands: Commands) {
    commands.spawn((
        StandardPiece::t(),
        GridPosition { col: 5, row: 0 },
        Color(1f32, 0f32, 0f32),
    ));
}

fn render_piece(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    block_size: Res<BlockSize>,
    query_pieces: Query<(&StandardPiece, &GridPosition, &Color)>,
    window: Single<&Window>,
    grid: Single<&Grid>,
) {
    let block_size = block_size.0;
    // FIXME: this should probably take into account the current window size
    let grid_width = block_size * grid.cols as f32;
    //let grid_height = block_size.0 * grid.rows as f32;

    let initial_x = (window.width() / 2f32) - grid_width;
    let initial_y = 0f32; // a little bit of a margin on top

    for (piece, grid_position, color) in &query_pieces {
        dbg!(&grid_position);
        for block_offset in piece.rotations.cur_offsets().offset_positions.as_ref() {
            let block_shape = meshes.add(Rectangle::new(block_size, block_size));
            // FIXME: maybe I need to add block_size to x, it depends in which direction the Rectangle is drawn towards in the x-axis.
            let x = initial_x + ((grid_position.col as i32 + block_offset.0) as f32 * block_size);
            let y = initial_y
                - ((grid_position.row as i32 + block_offset.1) as f32 * block_size)
                - block_size; // minus block_size because it draws the rectangle upwards
            commands.spawn((
                Mesh2d(block_shape),
                MeshMaterial2d(materials.add(bevy::color::Color::hsl(color.0, color.1, color.2))),
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

fn main() {
    App::new()
        .insert_resource(BlockSize(10f32))
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (setup, add_grid, (add_initial_pieces, render_piece).chain()),
        )
        .run();
}
