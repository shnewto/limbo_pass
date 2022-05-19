use bevy::prelude::*;

use crate::asset;
use crate::form;
use crate::fsm;
use crate::menu;
use crate::scenes;
use crate::theme;

pub struct LimboPlugin;

impl Plugin for LimboPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(fsm::Fsm::Loading)
            .add_plugin(asset::LoadingPlugin)
            .add_plugin(menu::MenuPlugin)
            .add_plugin(theme::ThemePlugin)
            .add_plugin(scenes::ScenePlugin)
            .add_plugin(form::FormPlugin);
    }
}
