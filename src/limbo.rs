use bevy::prelude::*;

use crate::assets::AssetsPlugin;
use crate::form::FormPlugin;
use crate::fsm::Fsm;
use crate::loading_screen::LoadingScreenPlugin;
use crate::setup::SetupPlugin;
use crate::theme::ThemePlugin;

pub struct LimboPlugin;

impl Plugin for LimboPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(Fsm::LoadingScreen)
            .add_plugin(LoadingScreenPlugin)
            .add_plugin(AssetsPlugin)
            .add_plugin(ThemePlugin)
            .add_plugin(SetupPlugin)
            .add_plugin(FormPlugin);
    }
}
