use bevy::{
  camera:: Viewport,
  color::palettes::{
    basic::WHITE,
    css::{ GREEN, RED },
  },
  math::ops::powf,
  prelude::*,
};

mod board;

const GAME_WINDOW_TITLE: &str = "Grand Strategy Chess Window";
const GAME_TITLE: &str = "Grand Strategy Chess";
const X_EXTENT: f32 = 1000.;
const Y_EXTENT: f32 = 150.;
const THICKNESS: f32 = 5.0;


fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // TODO: Move to "board.rs" along with other todos tagged "BOARD"
  const board_size: u8 = 8;

  commands.spawn(Camera2d);

  let tiles: Vec<Handle<Mesh>>;

  for i in 0..(board_size - 1) {
    for j in 0..(board_size - 1) {
      tiles.push(meshes.add(Rectangle::new(50., 50.)));
    }
  }

  let num_tiles = board_size ^ 2;

  // TODO: BOARD
  for (i, tile) in 0..(board_size - 1) {
    for j in 0..(board_size - 1) {
      // TODO: Move into a shape object/array
      let color = Color::hsl(360. * (i * 10 + j) as f32 / num_tiles as f32, 0.95, 0.7);

      commands.spawn((
        Mesh2d(),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
          // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
          -X_EXTENT / 2. + (i * 10 + j) as f32 / (num_tiles - 1) as f32 * X_EXTENT,
          -Y_EXTENT / 2.,
          0.0,
        ),
      ));
      
    }
  }
}