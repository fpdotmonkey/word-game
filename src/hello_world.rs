use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct HelloWorld {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl GodotExt for HelloWorld {
    fn init(base: Base<Node>) -> Self {
        HelloWorld { base }
    }

    fn ready(&mut self) {
        godot_print!("Hello, World!");
    }
}
