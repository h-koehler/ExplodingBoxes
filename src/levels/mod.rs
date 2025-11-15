use bevy::prelude::*;

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
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Bad(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Bad(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Bad(vec!["simple"]),
                SpawnItem::Bad(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
                SpawnItem::Bad(vec!["simple"]),
                SpawnItem::Good(vec!["simple"]),
            ],
        }),
        _ => todo!(),
    }
}

pub(super) fn register(app: &mut App) {
    app.add_systems(Update, setup_level.run_if(resource_changed::<Level>))
        .insert_resource(Level::One);
}
