use bevy::{color::palettes::css, prelude::*};

use crate::boxes::spawn::{SpawnItem, SpawnList};

pub mod advance;

#[derive(Resource, Clone, Copy)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Level {
    pub fn advance(&mut self) -> bool {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => return true,
        };

        false
    }
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
        Level::Two => commands.insert_resource(SpawnList {
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
    advance::register(app);

    app.add_systems(Update, setup_level.run_if(resource_changed::<Level>))
        .insert_resource(Level::One);
}
