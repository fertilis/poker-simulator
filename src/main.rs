use bevy::{prelude::*, window::WindowResolution};
use graphics::TableUiPlugin;
use table::{states::PausedState, TablePlugin};

mod graphics;
mod table;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Let's play!".to_string(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resolution: WindowResolution::new(800.0, 600.0).with_scale_factor_override(1.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            TablePlugin,
            TableUiPlugin,
        ))
        .insert_state(PausedState::Paused)
        .run();
}

//fn main() {
//    App::new()
//        .add_plugins(DefaultPlugins.set(WindowPlugin {
//            primary_window: Some(Window {
//                title: "Let's play!".to_string(),
//                position: WindowPosition::Centered(MonitorSelection::Primary),
//                resolution: WindowResolution::new(800.0, 600.0).with_scale_factor_override(1.0),
//                ..Default::default()
//            }),
//            ..Default::default()
//        }))
//        .add_systems(Startup, (spawn_camera, spawn_ball).chain())
//        .add_systems(
//            Update,
//            (move_ball, project_positions.after(move_ball)).chain(),
//        )
//        .run();
//}
//
//#[derive(Component)]
//struct MainCamera;
//
//fn spawn_camera(mut commands: Commands) {
//    commands.spawn((Camera2dBundle::default(), MainCamera));
//}
//
//#[derive(Component)]
//struct Position(Vec2);
//
//#[derive(Component)]
//struct Velocity(Vec2);
//
//#[derive(Component)]
//struct Ball;
//
//#[derive(Bundle)]
//struct BallBundle {
//    position: Position,
//    velocity: Velocity,
//    ball: Ball,
//}
//
//impl BallBundle {
//    fn new(x: f32, y: f32) -> Self {
//        Self {
//            position: Position(Vec2::new(x, y)),
//            velocity: Velocity(Vec2::new(1.0, 1.0)),
//            ball: Ball,
//        }
//    }
//}
//
//fn spawn_ball(
//    mut commands: Commands,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<ColorMaterial>>,
//) {
//    let shape = Mesh::from(Circle::new(10.0));
//    let color = ColorMaterial::from(Color::rgb(1.0, 1.0, 0.0));
//
//    let mesh_handle = meshes.add(shape);
//    let material_handle = materials.add(color);
//
//    commands.spawn((
//        BallBundle::new(0.0, 0.0),
//        MaterialMesh2dBundle {
//            mesh: mesh_handle.into(),
//            material: material_handle,
//            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
//            ..Default::default()
//        },
//    ));
//}
//
//fn project_positions(mut query: Query<(&Position, &mut Transform)>) {
//    for (position, mut transform) in query.iter_mut() {
//        transform.translation = position.0.extend(0.0);
//    }
//}
//
//fn move_ball(mut q_ball: Query<(&mut Position, &Velocity), With<Ball>>) {
//    for (mut position, velocity) in q_ball.iter_mut() {
//        position.0 += velocity.0;
//    }
//}
