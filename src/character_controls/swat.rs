use bevy::prelude::*;

use crate::{
    boxes::{BoxKicked, GameBox, GoodBox},
    character_controls::{Character, Velocity},
    custom_utils::GameState,
};

#[derive(Component)]
pub struct Swatted;

const SWAT_VEL: f32 = 1000.0;

fn on_swat(
    q_player: Query<&Transform, With<Character>>,
    mut q_swatted: Query<(&mut Velocity, &Transform, Has<GoodBox>), Added<Swatted>>,
    mut message_writer: MessageWriter<BoxKicked>,
) {
    for (mut vel, swatted_trans, is_good) in q_swatted.iter_mut() {
        let player_transform = q_player.single().expect("no player trans");
        vel.linear_velocity += (swatted_trans.translation - player_transform.translation)
            .normalize_or(Vec3::Y)
            .xy()
            * SWAT_VEL;
        message_writer.write(if is_good {
            BoxKicked::GoodBox
        } else {
            BoxKicked::BadBox
        });
    }
}

fn boss_swat(
    mut q_player_velocity: Query<&mut Velocity, Added<DidBadSwat>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for mut vel in q_player_velocity.iter_mut() {
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/belt.ogg")));

        vel.linear_velocity.x = SWAT_VEL;
        vel.linear_velocity.y = SWAT_VEL;
    }
}

#[derive(Component)]
pub struct DidBadSwat;

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
        .filter(|(dist, _, _)| *dist < 50.0 * 50.0)
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
    app.add_systems(
        Update,
        (
            (find_near_box, swat, on_swat).run_if(in_state(GameState::Running)),
            boss_swat,
        )
            .chain(),
    );
}
