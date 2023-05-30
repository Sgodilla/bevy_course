//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::Projection,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui::{self, panel},
    EguiContexts, EguiPlugin,
};

#[derive(Default, Resource)]
struct PanelDimensions {
    top_option_bar: f32,
    left_code_editor: f32,
    bottom_logger: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

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
        .add_plugin(EguiPlugin)
        .insert_resource(ClearColor(Color::hex("20232A").unwrap()))
        .init_resource::<PanelDimensions>()
        .insert_resource(CameraSettings {
            move_sensitivity: 1.0,
            rotate_sensitivity: 2.0,
            zoom_sensitivity: 1.0,
        })
        .add_startup_systems((spawn_fly_camera, spawn_meshes, spawn_lights))
        .add_systems((translate_camera, rotate_camera, zoom_camera))
        .add_system(editor_ui)
        .run();
}

// Setup a fly camera
fn spawn_fly_camera(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));
    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
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

fn editor_ui(mut contexts: EguiContexts, mut panel_dimensions: ResMut<PanelDimensions>) {
    let context = contexts.ctx_mut();

    // panel_dimensions.top_option_bar = 10.0;
    // egui::TopBottomPanel::top("top_option_bar")
    //     .resizable(false)
    //     .show(context, |ui| {
    //         ui.label("Top Options Bar");
    //         ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
    //     })
    //     .response
    //     .rect
    //     .set_height(panel_dimensions.top_option_bar);

    panel_dimensions.top_option_bar = egui::TopBottomPanel::top("top_panel")
        .resizable(false)
        .default_height(20.0)
        .show(context, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        })
        .response
        .rect
        .height();

    panel_dimensions.left_code_editor = egui::SidePanel::left("left_code_editor")
        .resizable(true)
        .show(context, |ui| {
            ui.label("Left Resizable Code Editor");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn update_camera_transform(
    panel_dimensions: Res<PanelDimensions>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = panel_dimensions.left_code_editor / window.width();
    let right_taken = 0.0;
    let top_taken = panel_dimensions.top_option_bar / window.height();
    let bottom_taken = 0.0;
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}
