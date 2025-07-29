use crate::block;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::{
    draw::Stroke,
    entity::ShapeBundle,
    prelude::GeometryBuilder,
    shapes::{self, RectangleOrigin},
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InitialGridPosition(5, 0))
            .add_systems(Startup, (spawn_grid, render_grid).chain());
    }
}

#[derive(Debug, Resource, Copy, Clone)]
pub struct InitialGridPosition(u32, u32);

#[derive(Debug, Component, Copy, Clone)]
pub struct GridPosition {
    pub col: u32,
    pub row: u32,
}

impl From<InitialGridPosition> for GridPosition {
    fn from(value: InitialGridPosition) -> Self {
        Self {
            col: value.0,
            row: value.1,
        }
    }
}

#[derive(Component)]
pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub position: Transform,
}

fn spawn_grid(mut commands: Commands, block_size: Res<block::BlockSize>, window: Single<&Window>) {
    let block_size = block_size.0;
    let grid_cols = 10;
    let grid_width = grid_cols as f32 * block_size;
    let x = (window.width() / 2f32) - (grid_width / 2f32);
    let margin_y = -block_size;

    let grid = Grid {
        cols: grid_cols,
        rows: 20,
        position: Transform::from_xyz(x, margin_y, -1f32),
    };
    commands.spawn((grid, Stroke::color(WHITE)));
}

fn render_grid(
    mut commands: Commands,
    block_size: Res<block::BlockSize>,
    grid: Single<(&Grid, &Stroke)>,
) {
    let (grid, stroke) = *grid;
    let block_size = block_size.0;
    let grid_width = grid.cols as f32 * block_size;
    let grid_height = grid.rows as f32 * block_size;

    let shape = shapes::Rectangle {
        extents: Vec2::new(grid_width, grid_height),
        origin: RectangleOrigin::TopLeft,
        ..default()
    };

    let position = grid.position.translation;
    let outline = ShapeBundle {
        path: GeometryBuilder::build_as(&shape),
        transform: Transform::from_xyz(position.x, position.y, -1f32),
        ..default()
    };
    commands.spawn((outline, Stroke::new(stroke.color, 2.0)));
}
