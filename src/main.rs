use bevy::prelude::*;
use grid::Grid;
mod grid;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_grid, add_camera))
        .add_systems(FixedUpdate, move_dot)
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn create_grid(mut commands: Commands) {
    let grid = Grid::from(7, 500);

    commands.spawn_batch(grid.build());
}
fn move_dot(mut commands: Commands) {
    let grid = Grid::from(7, 500);
    commands.spawn_batch(grid.update(1, 0, Color::BLACK));
    commands.spawn_batch(grid.update(1, 0, Color::WHITE));
    commands.spawn_batch(grid.update(0, 0, Color::BLACK));
    commands.spawn_batch(grid.update(1, 0, Color::WHITE));
}
