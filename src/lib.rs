use godot::prelude::*;

mod hello_world;

pub struct WordGame;

#[gdextension]
unsafe impl ExtensionLibrary for WordGame {}
