use bevy::{
  camera::Viewport,
  color::palettes::{
    basic::WHITE,
    css::{ GREEN, RED },
  },
  math::ops::powf,
  prelude::*,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(FixedUpdate, controls)
    .add_systems(PostUpdate, draw_cursor.after(TransformSystems::Propagate))
    .run();
}

fn draw_cursor() { }

fn controls() { }

fn setup() { }








