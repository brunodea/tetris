use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[derive(Resource)]
struct DebugLinesEnabled(bool);

#[derive(Resource)]
struct BlockSize(f32);

// FIXME: this can probably be removed
#[derive(Resource)]
struct InitialGridPosition(u32, u32);

#[derive(Debug, Component, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
struct OffsetPosition(i32, i32, usize);
impl From<(i32, i32, usize)> for OffsetPosition {
    fn from((col, row, idx): (i32, i32, usize)) -> Self {
        OffsetPosition(col, row, idx)
    }
}

struct PieceOffsets<const NUM_OF_BLOCKS: usize> {
    offset_positions: [OffsetPosition; NUM_OF_BLOCKS],
}

impl<const NUM_OF_BLOCKS: usize> PieceOffsets<NUM_OF_BLOCKS> {
    pub fn new(positions: [(i32, i32); NUM_OF_BLOCKS]) -> Self {
        let mut offset_positions = [OffsetPosition(0i32, 0i32, 0usize); NUM_OF_BLOCKS];
        for (idx, (x, y)) in positions.iter().enumerate() {
            offset_positions[idx] = OffsetPosition(*x, *y, idx);
        }
        PieceOffsets { offset_positions }
    }
}

#[derive(Component)]
struct PieceRotations<const NUM_OF_BLOCKS: usize, const NUM_OF_ROTATIONS: usize> {
    rotations_offsets: [PieceOffsets<NUM_OF_BLOCKS>; NUM_OF_ROTATIONS],
    current: usize,
}

impl<const NUM_OF_BLOCKS: usize, const NUM_OF_ROTATIONS: usize>
    PieceRotations<NUM_OF_BLOCKS, NUM_OF_ROTATIONS>
{
    pub fn new(rotations_offsets: [PieceOffsets<NUM_OF_BLOCKS>; NUM_OF_ROTATIONS]) -> Self {
        Self {
            rotations_offsets,
            current: 0usize,
        }
    }

    pub fn cur_offsets(&self) -> &PieceOffsets<NUM_OF_BLOCKS> {
        self.rotations_offsets
            .get(self.current)
            .expect("invalid value for current rotations offsets")
    }

    pub fn rotate(&mut self) {
        self.current = (self.current + 1) % NUM_OF_ROTATIONS;
    }

    pub fn rotation(&self) -> usize {
        self.current
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

#[derive(Component)]
struct BlockIdx(usize);

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

fn toggle_debug_lines_enabled(
    mut enabled: ResMut<DebugLinesEnabled>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        enabled.0 = !enabled.0;
    }
}

fn grid_debug_lines(
    enabled: Res<DebugLinesEnabled>,
    block_size: Res<BlockSize>,
    grid: Single<(&Grid, &Position)>,
    mut gizmos: Gizmos,
) {
    if enabled.0 {
        let (grid, position) = *grid;
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

fn spawn_t(mut commands: Commands, grid_position: GridPosition) {
    info!("Spawning T at {:?}", grid_position);
    let (shape, rotations) = Shape::t();
    commands.spawn((
        shape,
        rotations,
        grid_position,
        Fill::color(RED),
        ActivePiece,
    ));
}

fn initial_spawn_t(commands: Commands, initial_grid_position: Res<InitialGridPosition>) {
    spawn_t(
        commands,
        GridPosition {
            col: initial_grid_position.0,
            row: initial_grid_position.1,
        },
    );
}

fn block_transform(
    block_offset: &OffsetPosition,
    grid_canvas_position: &Position,
    grid_position: &GridPosition,
    block_size: &BlockSize,
) -> Transform {
    let x = grid_canvas_position.x
        + ((grid_position.col as i32 + block_offset.0) as f32 * block_size.0);
    let y = grid_canvas_position.y
        - ((grid_position.row as i32 + block_offset.1) as f32 * block_size.0)
        - block_size.0; // minus block_size because it draws the rectangle upwards
    Transform::from_xyz(x, y, 0f32)
}

fn render_active_piece(
    mut commands: Commands,
    block_size: Res<BlockSize>,
    query_pieces: Query<(&StandardPieceRotations, &GridPosition, &Fill), With<ActivePiece>>,
    grid_canvas_position: Single<&Position, With<Grid>>,
) {
    for (rotations, piece_grid_position, color) in &query_pieces {
        for block_offset in rotations.cur_offsets().offset_positions.as_ref() {
            let shape = shapes::Rectangle {
                extents: Vec2::new(block_size.0, block_size.0),
                origin: RectangleOrigin::TopLeft,
                ..default()
            };

            let outline = ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: block_transform(
                    &block_offset,
                    &grid_canvas_position,
                    &piece_grid_position,
                    &block_size,
                ),
                ..default()
            };

            commands.spawn((
                outline,
                Fill::color(color.color),
                Stroke::new(BLACK, 1.0),
                BlockIdx(block_offset.2),
                ActivePiece,
            ));
        }
    }
}

fn rotate_piece(
    mut single_piece: Single<(&mut StandardPieceRotations, &GridPosition), With<ActivePiece>>,
    mut query_block: Query<(&mut Transform, &BlockIdx), With<ActivePiece>>,
    grid_canvas_position: Single<&Position, With<Grid>>,
    block_size: Res<BlockSize>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        single_piece.0.rotate();
        let cur_offsets = single_piece.0.cur_offsets();
        let grid_position = single_piece.1;

        for (mut transform, idx) in &mut query_block {
            let block_offset = cur_offsets.offset_positions[idx.0];
            *transform = block_transform(
                &block_offset,
                &grid_canvas_position,
                &grid_position,
                &block_size,
            );
        }
    }
}

#[derive(Resource)]
struct GravityTimer(Timer);

#[derive(Resource)]
struct GravitySpeed(f32);

fn apply_gravity(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GravityTimer>,
    mut single_piece: Single<
        (Entity, &StandardPieceRotations, &mut GridPosition),
        With<ActivePiece>,
    >,
    mut query_block: Query<&mut Transform, With<ActivePiece>>,
    block_size: Res<BlockSize>,
    grid: Single<&Grid>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        single_piece.2.row += 1;
        for mut transform in &mut query_block {
            transform.translation.y -= block_size.0;
        }
        let mut lowest_row_offset = None;
        for offset in single_piece.1.cur_offsets().offset_positions {
            let block_row_offset = offset.2;
            if lowest_row_offset.is_none() {
                lowest_row_offset = Some(block_row_offset);
            } else if let Some(lowest) = lowest_row_offset {
                if block_row_offset < lowest {
                    lowest_row_offset = Some(block_row_offset);
                }
            }
        }
        if let Some(lowest_row_offset) = lowest_row_offset {
            // check if the bottom block is touching the bottom of the board
            let piece_row = single_piece.2.row + lowest_row_offset as u32;
            dbg!(&piece_row);
            dbg!(&grid.rows);
            if piece_row >= grid.rows {
                warn!("Removed active piece!");
                commands.entity(single_piece.0).remove::<ActivePiece>();
            }
        }
    }
}

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

// TODO: add a "GameState" resource or something so we can have "Paused" and we pause gravity and everything
fn main() {
    App::new()
        .insert_resource(BlockSize(30f32))
        .insert_resource(DebugLinesEnabled(false))
        .insert_resource(InitialGridPosition(5, 0))
        .insert_resource(GravityTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(GravitySpeed(1.0))
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_systems(
            Startup,
            (
                setup,
                (spawn_grid, render_grid).chain(),
                (initial_spawn_t, render_active_piece).chain(),
            ),
        )
        .add_systems(Update, (increase_gravity, rotate_piece))
        .add_systems(Update, (toggle_debug_lines_enabled, grid_debug_lines))
        .run();
}
