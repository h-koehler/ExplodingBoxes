use bevy::prelude::*;

pub mod boxes;
pub mod character_controls;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    character_controls::register(&mut app);
    boxes::register(&mut app);

    app.run();
}
