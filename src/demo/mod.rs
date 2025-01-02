//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

pub mod level;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
    ));
}
