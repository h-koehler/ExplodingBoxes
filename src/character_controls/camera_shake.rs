use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct CameraShake {
    pub time: f32,
    pub intensity: f32,
    pub og_time: f32,
}

impl CameraShake {
    pub fn new(duration: Duration, intensity: f32) -> Self {
        Self {
            intensity,
            og_time: duration.as_secs_f32(),
            time: duration.as_secs_f32(),
        }
    }
}

fn shake_cam(
    time: Res<Time>,
    mut q_cam: Query<(Entity, &mut Transform, &mut CameraShake)>,
    mut commands: Commands,
) {
    for (ent, mut trans, mut cam_shake) in q_cam.iter_mut() {
        cam_shake.time -= time.delta_secs();
        if cam_shake.time <= 0.0 {
            commands.entity(ent).remove::<CameraShake>();
            trans.translation = Vec3::ZERO;
        } else {
            let intensity = (cam_shake.time / cam_shake.og_time) * cam_shake.intensity;
            trans.translation = Vec3::new(
                intensity * rand::random::<f32>(),
                intensity * rand::random::<f32>(),
                trans.translation.z,
            );
        }
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, shake_cam);
}
