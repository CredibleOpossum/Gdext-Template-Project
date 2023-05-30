use godot::prelude::*;

mod main_behavior;

struct Unmanaged;

#[gdextension]
unsafe impl ExtensionLibrary for Unmanaged {}
