use godot::prelude::*;

mod player;
mod enemy;
mod game_camera;
mod sword_ability;
mod sword_ability_controller;

struct RustyVampire;

#[gdextension]
unsafe impl ExtensionLibrary for RustyVampire {}
