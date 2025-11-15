use bevy::prelude::*;

use crate::{
    boxes::GameBox,
    character_controls::{Character, Velocity},
};

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

fn find_near_box(
    q_player: Query<&Transform, With<Character>>,
    q_box: Query<(Entity, &Transform), With<GameBox>>,
    q_selected: Query<Entity, With<NearBox>>,
    mut commands: Commands,
) {
    let trans = q_player.single().expect("no player (dies)");

    let Some((_, new_near, _)) = q_box
        .iter()
        .map(|(e, t)| (t.translation.distance_squared(trans.translation), e, t))
        .filter(|(dist, _, _)| *dist < 50.0)
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    else {
        if let Ok(ent) = q_selected.single() {
            commands.entity(ent).remove::<NearBox>();
        }
        return;
    };

    if let Ok(old_near) = q_selected.single() {
        if old_near != new_near {
            commands.entity(old_near).remove::<NearBox>();
            commands.entity(new_near).insert(NearBox);
        }
    } else {
        commands.entity(new_near).insert(NearBox);
    }
}

fn swat(
    mut commands: Commands,
    inputs: Res<ButtonInput<KeyCode>>,
    q_near: Query<Entity, With<NearBox>>,
) {
    if inputs.just_pressed(KeyCode::Space) {
        let Ok(near) = q_near.single() else {
            return;
        };
        commands.entity(near).insert(Swatted);
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, (find_near_box, swat, on_swat).chain());
}
