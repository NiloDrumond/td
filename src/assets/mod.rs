use bevy::prelude::*;

use crate::config::SysLabel;

pub use self::sprites::SpriteSheets;
use self::sprites::{load_map_sheet, load_enemy_sheet};

mod sprites;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpriteSheets::default())
            .add_startup_system_set_to_stage(
                StartupStage::PreStartup,
                SystemSet::new()
                    .label(SysLabel::LoadAssets)
                    .with_system(load_enemy_sheet)
                    .with_system(load_map_sheet),
            );
    }
}
