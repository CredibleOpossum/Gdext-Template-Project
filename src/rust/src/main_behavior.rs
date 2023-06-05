use godot::engine::{Engine, Node2D, Node2DVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainBehavior {
    #[base]
    base: Base<Node2D>,
}

#[cfg(not(debug_assertions))]
macro_rules! crash_if_release {
    () => {
        #[allow(unconditional_recursion)]
        fn recurse() {
            recurse();
        }
        recurse();
    };
}

#[cfg(debug_assertions)]
macro_rules! crash_if_release {
    () => {
        // We are in debug... let's not crash.
    };
}

#[godot_api]
impl Node2DVirtual for MainBehavior {
    fn init(base: Base<Node2D>) -> Self {
        MainBehavior { base }
    }
    fn process(&mut self, _delta: f64) {
        if Engine::singleton().is_editor_hint() {
            // let's make reverse engineers have a little work for their fun :), recursion makes a hard to debug error.
            // if we are in release mode, there's no reason the editor should be open, this would only occur when someone reverse
            // engineering the project uses the built binary (the only one that should be available to them) to open the project in the editor
            crash_if_release!();
            // otherwise, let's just be safe and return early so this node does nothing until we actually start the project
            return;
        }
        let input = Input::singleton();
        if input.is_action_just_pressed("input_exit".into(), false) {
            self.get_tree().unwrap().quit(0);
        }
    }
}
