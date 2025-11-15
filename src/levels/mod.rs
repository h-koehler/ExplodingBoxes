use bevy::{color::palettes::css, prelude::*};

use crate::boxes::spawn::{SpawnItem, SpawnList};

#[derive(Resource, Clone, Copy)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
}

fn setup_level(mut commands: Commands, level: Res<Level>) {
    match *level {
        Level::One => commands.insert_resource(SpawnList {
            entries: vec![
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Bad(vec![("simple", css::RED)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Bad(vec![("simple", css::RED)]),
                SpawnItem::Bad(vec![("simple", css::RED)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
                SpawnItem::Good(vec![("simple", css::GREEN)]),
            ],
        }),
        _ => todo!(),
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, setup_level.run_if(resource_changed::<Level>))
        .insert_resource(Level::One);
}
