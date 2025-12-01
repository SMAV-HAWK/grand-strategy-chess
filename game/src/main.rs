use bevy::{
  camera:: Viewport,
  color::palettes::{
    basic::WHITE,
    css::{ GREEN, RED },
  },
  math::ops::powf,
  prelude::*,
};

const GAME_WINDOW_TITLE: &str = "Grand Strategy Chess Window";
const GAME_TITLE: &str = "Grand Strategy Chess";

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: GAME_WINDOW_TITLE.into(),
        name: Some(GAME_TITLE.into()),
        ..default()
      }),
      ..default()
    }))
    .add_systems(Startup, setup)
    .add_systems(FixedUpdate, controls)
    .add_systems(PostUpdate, draw_cursor.after(TransformSystems::Propagate))
    .run();
}

fn draw_cursor(
  camera_query: Single<(&Camera, &GlobalTransform)>,
  window: Single<&Window>,
  mut gizmos: Gizmos,
) { 
  let (camera, camera_transfrom) = *camera_query;

  if let Some(cursor_position) = window.cursor_position() 
    && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transfrom, cursor_position)
    && let Ok(viewport_check) = camera.world_to_viewport(camera_transfrom, world_pos.extend(0.0)) 
    && let Ok(world_check) = camera.viewport_to_world_2d(camera_transfrom, viewport_check.xy()) 
  {
    gizmos.circle_2d(world_pos, 10., WHITE);
    gizmos.circle_2d(world_check, 8., RED);
  }
}

fn controls(
  camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
  window: Single<&Window>,
  input: Res<ButtonInput<KeyCode>>,
  time: Res<Time<Fixed>>
) { 
  let (mut camera, mut transform, mut projection) = camera_query.into_inner();

  let fspeed = 600.0 * time.delta_secs();
  let uspeed = fspeed as u32;
  let window_size = window.resolution.physical_size();

  if input.pressed(KeyCode::ArrowUp) { 
    transform.translation.y += fspeed;
  }
  if input.pressed(KeyCode::ArrowDown) {
    transform.translation.y -= fspeed;
  }
  if input.pressed(KeyCode::ArrowLeft) {
    transform.translation.x -= fspeed;
  }
  if input.pressed(KeyCode::ArrowRight) {
    transform.translation.x += fspeed;
  }

  if let Projection::Orthographic(projection2d) = &mut *projection {
    if input.pressed(KeyCode::Comma) {
      projection2d.scale *= powf(4.0f32, time.delta_secs());
    }
    if input.pressed(KeyCode::Period) {
      projection2d.scale *= powf(0.25f32, time.delta_secs());
    }
  }

  if let Some(viewport) = camera.viewport.as_mut() {
    if input.pressed(KeyCode::KeyW) && !input.pressed(KeyCode::KeyS) {
      viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
    }
    if input.pressed(KeyCode::KeyS) && !input.pressed(KeyCode::KeyW) {
      viewport.physical_position.y += uspeed;
    }
    if input.pressed(KeyCode::KeyA) && !input.pressed(KeyCode::KeyD) {
      viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
    }
    if input.pressed(KeyCode::KeyD) && !input.pressed(KeyCode::KeyA) {
      viewport.physical_position.x += uspeed;
    }

    viewport.physical_position = viewport.physical_position.min(window_size - viewport.physical_size);

    if input.pressed(KeyCode::KeyI) {
        viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
    }
    if input.pressed(KeyCode::KeyK) {
        viewport.physical_size.y += uspeed;
    }
    if input.pressed(KeyCode::KeyJ) {
        viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
    }
    if input.pressed(KeyCode::KeyL) {
        viewport.physical_size.x += uspeed;
    }

    viewport.physical_size = viewport.physical_size.min(window_size - viewport.physical_position).max(UVec2::new(20, 20));
  }
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  window: Single<&Window>
) { 
  let window_size = window.resolution.physical_size().as_vec2();
  
  commands.spawn((
    Camera2d,
    Camera {
      viewport: Some(Viewport {
        physical_position: (window_size * 0.125).as_uvec2(), 
        physical_size: window_size.as_uvec2(), 
        ..default()
      }),
      ..default()
    },
  ));

  commands.spawn((
    Text::new(
      "Move the mouse to see the circle follow your cursor.\n\
        Use the arrow keys to move the camera.\n\
        Use the comma and period keys to zoom in and out.\n\
        Use the WASD keys to move the viewport.\n\
        Use the IJKL keys to resize the viewport.",
    ),
    Node {
      position_type: PositionType::Absolute,
      top: px(12),
      left: px(12),
      ..default()
    }
  ));
  
  commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(40.0, 20.0))),
    MeshMaterial2d(materials.add(Color::linear_rgb(0.01, 0.01, 0.01))),
  ));  
  
  commands.spawn((
    Mesh2d(meshes.add(Rectangle::new(400.0, 200.0))),
    MeshMaterial2d(materials.add(Color::from(GREEN))),
    Transform::from_translation(Vec3::new(0.0,0.0,-200.0)),
    Node {
      position_type: PositionType::Absolute,
      top: px(12),
      left: px(12),
      ..default()
    }
  ));
}








