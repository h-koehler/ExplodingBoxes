use bevy::{color::palettes::css, prelude::*};

use crate::{
    boxes::spawn::{BoxAddOns, SpawnItem, SpawnList},
    ui::{BadAttributes, UIBad},
};

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
        Level::One => {
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("circle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![BadAttributes::Color(css::RED)],
            });
        }
        _ => todo!(),
    }
}

pub(super) fn register(app: &mut App) {
    advance::register(app);

    app.add_systems(Update, setup_level.run_if(resource_changed::<Level>))
        .insert_resource(Level::One);
}
