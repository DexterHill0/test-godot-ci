use godot::{classes::Engine, prelude::*};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainNode {
    base: Base<Node>,
}

pub fn get_main_node() -> Gd<MainNode> {
    Engine::singleton()
        .get_main_loop()
        .unwrap()
        .cast::<SceneTree>()
        .get_first_node_in_group("main")
        .unwrap()
        .cast::<MainNode>()
}

#[godot_api]
impl INode for MainNode {
    fn ready(&mut self) {
        let mut main_node = get_main_node();

        godot_print!("got main node: {main_node:?}");

        let mut scene = load::<PackedScene>("res://scenes/Test.tscn")
            .instantiate()
            .unwrap();

        main_node.add_child(&scene);
    }
}
