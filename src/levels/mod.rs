use bevy::{color::palettes::css, prelude::*};

use crate::{
    boxes::spawn::{BoxAddOns, SpawnItem, SpawnList},
    ui::{BadAttributes, UIBad},
};

pub mod advance;
pub mod level_select;

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

        true
    }
}

pub fn setup_level(mut commands: Commands, level: Res<Level>) {
    match *level {
        Level::One => {
            // Bad: Red, Good: Green, Grey (20)
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
                additional_text: vec![
                    "Press SPACE to swat all explosive boxes!".into(),
                    "See Left Guide for Explosive Box Types".into(),
                ],
            });
        }
        Level::Two => {
            // Introoduce triangles (25)
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![
                    BadAttributes::Color(css::RED),
                    BadAttributes::Symbol("triangle".into()),
                ],
                additional_text: vec!["Two Wrongs Make a Right".into()],
            });
        }
        Level::Three => {
            // Introoduce square - every 4th square is bad, regardless of color (25)
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 3
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 1
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 4 - BAD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 3
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 1
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 4 - BAD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 3
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 1
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![
                    BadAttributes::Color(css::RED),
                    BadAttributes::Symbol("triangle".into()),
                ],
                additional_text: vec!["How Many Sides Does a Square Have?".into()],
                // "how many sides does a square have?"
            });
        }
        Level::Four => {
            // Introoduce circle - copy the outcome of the previous box (EX. red box is bad -> green circle is bad)
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("circle".into()),
                    )]), // COPY GOOD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 4 - BAD
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 3
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY GOOD
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 1
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("circle".into()))]), // COPY GOOD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 4 - BAD
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("circle".into()),
                    )]), // COPY GOOD
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 3
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("circle".into()))]), // COPY GOOD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 1
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("circle".into()),
                    )]), // COPY GOOD
                    SpawnItem::Good(vec![(
                        "simple",
                        css::RED,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![
                    BadAttributes::Color(css::RED),
                    BadAttributes::Symbol("triangle".into()),
                ],
                additional_text: vec!["Circles copy the previous box!".into()],
            });
        }
        Level::Five => {
            // Introoduce plus sign - copies BEHAVIOR of previous box
            commands.insert_resource(SpawnList {
                entries: vec![
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("plus".into()))]), // COPY SQUARE - 1
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("plus".into()))]), // COPY SQUARE - 4 - BAD
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("plus".into()))]), // COPY SQUARE - 3
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("plus".into()))]), // COPY SQUARE - 1
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 4
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("circle".into()))]), // COPY GOOD
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::new("plus".into()))]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREY,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::new("plus".into()))]), // 3
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::new("square".into()))]), // 2
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 1
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]),
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![("simple", css::GREEN, BoxAddOns::new("plus".into()))]), // COPY SQUARE - 4 -> BAD
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("square".into()))]), // 3
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("plus".into()))]), // COPY SQARE - 2
                    SpawnItem::Good(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("square".into()),
                    )]), // 1
                    SpawnItem::Good(vec![("simple", css::RED, BoxAddOns::new("circle".into()))]), // COPY GOOD
                    SpawnItem::Good(vec![("simple", css::GREY, BoxAddOns::default())]),
                    SpawnItem::Bad(vec![(
                        "simple",
                        css::GREEN,
                        BoxAddOns::new("triangle".into()),
                    )]),
                    SpawnItem::Bad(vec![("simple", css::GREY, BoxAddOns::new("circle".into()))]), // COPY BAD
                    SpawnItem::Bad(vec![("simple", css::RED, BoxAddOns::default())]),
                    SpawnItem::Good(vec![("simple", css::GREEN, BoxAddOns::default())]),
                ],
            });
            commands.insert_resource(UIBad {
                bad_attributes: vec![
                    BadAttributes::Color(css::RED),
                    BadAttributes::Symbol("triangle".into()),
                ],
                additional_text: vec![],
            });
        }
    }
}

pub(super) fn register(app: &mut App) {
    advance::register(app);

    app.add_systems(Update, setup_level.run_if(resource_changed::<Level>))
        .insert_resource(Level::One);
}
