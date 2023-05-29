//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

#[derive(Resource)]
struct CameraSettings {
    pub move_sensitivity: f32,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
}

#[derive(Component)]
struct FlyCamera {
    pub zoom_level: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        FlyCamera { zoom_level: 1.0 }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::hex("20232A").unwrap()))
        .insert_resource(CameraSettings {
            move_sensitivity: 1.0,
            rotate_sensitivity: 2.0,
            zoom_sensitivity: 1.0,
        })
        .add_startup_systems((spawn_fly_camera, spawn_meshes, spawn_lights))
        .add_systems((translate_camera, rotate_camera, zoom_camera))
        .run();
}

// Setup a fly camera
fn spawn_fly_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCamera { zoom_level: 1.0 },
    ));
}

/// set up a simple 3D scene
fn spawn_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::hex("282C34").unwrap().into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::hex("3EAC79").unwrap().into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn spawn_lights(mut commands: Commands) {
    // a single point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn translate_camera(
    time: Res<Time>,
    camera_settings: Res<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let local_rotation = camera_transform.rotation;
    let movement_sensitivty = camera_settings.move_sensitivity;
    let delta = time.delta_seconds()
        * movement_sensitivty
        * camera_transform.translation.distance(Vec3::ZERO);

    if keyboard_input.pressed(KeyCode::W) {
        camera_transform.translation += local_rotation * Vec3::NEG_Z * delta;
    } else if keyboard_input.pressed(KeyCode::S) {
        camera_transform.translation += local_rotation * Vec3::Z * delta;
    }

    if keyboard_input.pressed(KeyCode::A) {
        camera_transform.translation += local_rotation * Vec3::NEG_X * delta;
    } else if keyboard_input.pressed(KeyCode::D) {
        camera_transform.translation += local_rotation * Vec3::X * delta;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        camera_transform.translation += local_rotation * Vec3::NEG_Y * delta;
    } else if keyboard_input.pressed(KeyCode::Space) {
        camera_transform.translation += local_rotation * Vec3::Y * delta;
    }
}

fn rotate_camera(
    windows: Query<&Window>,
    camera_settings: Res<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut ev_motion: EventReader<MouseMotion>,
) {
    let window = windows.single();
    let mut camera_transform = camera_query.single_mut();
    let mut mouse_rotation = Vec2::ZERO;
    let rotation_sensitivity = camera_settings.rotate_sensitivity;

    // If left mouse button pressed track mouse motion
    if mouse_input.pressed(MouseButton::Left) {
        for ev in ev_motion.iter() {
            mouse_rotation += ev.delta;
        }
    }

    let theta_y: f32 = (mouse_rotation.y / window.height()).asin() * rotation_sensitivity;
    let theta_x = (mouse_rotation.x / window.width()).asin() * rotation_sensitivity;
    let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    pitch -= theta_y;
    yaw -= theta_x;
    camera_transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);

    ev_motion.clear();
}

fn zoom_camera(
    camera_settings: Res<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut ev_scroll: EventReader<MouseWheel>,
) {
    let mut camera_transform = camera_query.single_mut();
    let local_rotation = camera_transform.rotation;
    let mut scroll = 0.0;
    let scroll_sensitivity = camera_settings.zoom_sensitivity;

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    camera_transform.translation += local_rotation * Vec3::NEG_Z * scroll * scroll_sensitivity;
}
