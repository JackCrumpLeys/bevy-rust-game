use crate::camera::CameraModel;
use crate::prelude::*;
use crate::Camera;

pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraFocus {
    pub weight: f32,
}

// #[derive(Component)]
// pub struct CameraInterest {
//     pub weight: f32,
//     pub active: bool,
// }

impl Default for CameraFocus {
    fn default() -> Self {
        CameraFocus { weight: 1.0 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CameraData {
    /// The Isometric (Inside / RTS) camera model
    pub isometric: CameraModel,
    pub look_at: Vec3,
}

impl Default for CameraData {
    fn default() -> Self {
        CameraData {
            isometric: CameraModel::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(32.0, 34.0, 32.0)),
            look_at: Vec3::ZERO,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(determine_focus)
                .with_system(update_camera.after(determine_focus)),
        )
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_camera));
    }
}

/// set up a simple 3D scene (Orthographic example)
fn spawn_camera(mut commands: Commands) {
    // set up the camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 10.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Z);

    commands.insert_resource(CameraData::default());
    // camera
    commands.spawn_bundle(camera);
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}

fn update_camera(
    time: Res<Time>, mut data: ResMut<CameraData>, mut cameras: Query<&mut Transform, With<Camera>>,
) {
    data.isometric.update(time.delta_seconds());

    for mut transform in cameras.iter_mut() {
        *transform = Transform::from_translation(data.isometric.abs_camera_pos())
            .looking_at(data.isometric.lookat_pos(), Vec3::Z);
    }
}

fn determine_focus(mut data: ResMut<CameraData>, focus_on: Query<(&CameraFocus, &Transform)>) {
    let mut look_at = Vec3::ZERO;
    let mut _tot_weight = 0.0;

    let mut look_min: Option<Vec3> = None;
    let mut look_max: Option<Vec3> = None;

    // Calculate a weighted average of all interesting locations
    for (focus, transform) in focus_on.iter() {
        let loc = transform.translation;
        look_at += loc * focus.weight;
        _tot_weight += focus.weight;

        // Find the bounds of all look_at locations
        look_min = Some(if let Some(l_min) = look_min { l_min.min(loc) } else { loc });
        look_max = Some(if let Some(l_max) = look_max { l_max.max(loc) } else { loc });
    }

    // TODO: Use look_min and look_max to adjust zoom

    data.look_at = look_at;
    data.isometric.set_look(look_at);
}
