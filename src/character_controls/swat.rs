use bevy::prelude::*;

use crate::character_controls::{Character, Velocity};

#[derive(Component)]
pub struct Swatted;

const SWAT_VEL: f32 = 400.0;

fn on_swat(
    q_player: Query<&Transform, With<Character>>,
    mut q_swatted: Query<(&mut Velocity, &Transform), Added<Swatted>>,
) {
    for (mut vel, swatted_trans) in q_swatted.iter_mut() {
        let player = q_player.single().expect("no player trans");
        vel.linear_velocity += (swatted_trans.translation - player.translation)
            .normalize_or(Vec3::Y)
            .xy()
            * SWAT_VEL;
    }
}

#[derive(Component)]
pub struct NearBox;

// fn swat_box(q_box: Query<&Transform, With<Box>>) {
//     let Some((_, conv, t)) = q_conveyor
//         .iter()
//         .map(|(t, c)| (t.translation.distance_squared(trans.translation), c, t))
//         .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
//     else {
//         continue;
//     };
// }

pub(super) fn register(app: &mut App) {}
