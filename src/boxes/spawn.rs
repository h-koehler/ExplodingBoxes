use bevy::prelude::*;

pub enum SpawnItem {
    Bad(Vec<(&'static str, Srgba, BoxAddOns)>),
    Good(Vec<(&'static str, Srgba, BoxAddOns)>),
}

#[derive(Default)]
pub struct BoxAddOns {
    pub addons: Vec<String>
}

impl BoxAddOns {
    pub fn new (addon:String) -> Self {
        Self {
            addons:vec![addon]
        }
    }

    pub fn and (mut self, addon:String) -> Self {
        self.addons.push(addon);
        self
    }
}

#[derive(Resource, Default)]
pub struct SpawnList {
    pub entries: Vec<SpawnItem>,
}

pub(super) fn register(app: &mut App) {
    app.init_resource::<SpawnList>();
}
