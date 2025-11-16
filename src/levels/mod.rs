use bevy::{color::palettes::css, prelude::*};

use crate::{
    boxes::spawn::{BoxAddOns, SpawnItem, SpawnList},
    ui::{BadAttributes, UIBad},
};

pub mod advance;

#[derive(Resource, Clone, Copy, Debug)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Level {
    pub fn advance(&mut self) -> bool {
        *self = match self {
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
        Level::One => { // Bad: Red, Good: Green, Grey (20)
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![BadAttributes::Color(css::RED)],
            });
        }
        Level::Two => { // Introoduce triangles (25)
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![BadAttributes::Color(css::RED), BadAttributes::Symbol("triangle".into())],
                // add text that says "two wrongs make a right ;)"
            });
        }
        Level::Three => { // Introoduce square - every 4th square is bad, regardless of color (25)
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 1
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("square".into()))]), // 1
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 4 - BAD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 3
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("square".into()))]), // 1
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 4 - BAD 
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 3
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("square".into()))]), // 1
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![BadAttributes::Color(css::RED), BadAttributes::Symbol("square".into())],
                // "how many sides does a square have?"
            });
        }
        Level::Four => { // Introoduce circle - copy the behavior of the box in front of it -> DOES count as a square in the square sequence
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("triangle".into()))]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![BadAttributes::Color(css::RED), BadAttributes::Symbol("square".into())],
                // 
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
