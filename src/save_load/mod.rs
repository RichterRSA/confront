use bevy::prelude::*;
use moonshine_save::prelude::*;

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((SavePlugin, LoadPlugin))
            .add_systems(PreUpdate, save_default().into_file_on_request::<SaveRequest>())
            .add_systems(PreUpdate, load_from_file_on_request::<LoadRequest>());
    }
}

#[derive(Resource)]
pub struct SaveRequest(pub String);

impl SaveIntoFileRequest for SaveRequest {
    fn path(&self) -> &std::path::Path {
        self.0.as_ref()
    }
}

#[derive(Resource)]
pub struct LoadRequest(pub String);

impl LoadFromFileRequest for LoadRequest {
    fn path(&self) -> &std::path::Path {
        self.0.as_ref()
    }
}