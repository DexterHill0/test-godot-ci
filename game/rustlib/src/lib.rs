use godot::{
    classes::{EditorExportPlugin, EditorPlugin, Engine, IEditorExportPlugin, IEditorPlugin},
    prelude::*,
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(tool, init, base=EditorExportPlugin)]
struct BuildInfoPlugin {
    base: Base<EditorExportPlugin>,
}

#[godot_api]
impl IEditorExportPlugin for BuildInfoPlugin {
    fn customize_resource(
        &mut self,
        resource: Gd<Resource>,
        path: GString,
    ) -> Option<Gd<Resource>> {
        None
    }

    fn customize_scene(&mut self, scene: Gd<Node>, path: GString) -> Option<Gd<Node>> {
        None
    }

    fn get_customization_configuration_hash(&self) -> u64 {
        0
    }

    fn get_name(&self) -> GString {
        "BuildInfoPlugin".into()
    }

    fn export_begin(
        &mut self,
        features: PackedStringArray,
        is_debug: bool,
        path: GString,
        flags: u32,
    ) {
        self.base_mut().add_file(
            "build_number.txt",
            &PackedByteArray::from(b"example value"),
            false,
        );
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=EditorPlugin)]
struct RustEditorPlugin {
    base: Base<EditorPlugin>,
    build_info_plugin: Gd<BuildInfoPlugin>,
}

#[godot_api]
impl IEditorPlugin for RustEditorPlugin {
    fn enter_tree(&mut self) {
        let plugin = BuildInfoPlugin::new_gd();
        self.build_info_plugin = plugin.clone();
        self.base_mut().add_export_plugin(&plugin);
    }

    fn exit_tree(&mut self) {
        let plugin = self.build_info_plugin.clone();
        self.base_mut().remove_export_plugin(&plugin);
    }
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainNode {
    base: Base<Node>,

    scene_stack: Vec<Gd<Node>>,
}

pub fn get_main_node() -> Gd<MainNode> {
    Engine::singleton()
        .get_main_loop()
        .unwrap()
        .cast::<SceneTree>()
        .get_first_node_in_group("main testing")
        .unwrap()
        .cast::<MainNode>()
}

fn push_scene(file: impl AsRef<str>) {
    let file = file.as_ref();

    let scene = load::<PackedScene>(file).instantiate().unwrap();

    let mut main_node = get_main_node();

    godot_print!("got main node: {main_node:?}");

    if let Some(current) = main_node.get_child(0) {
        main_node.remove_child(&current);
        main_node.bind_mut().scene_stack.push(current);
    }

    main_node.add_child(&scene);
}

#[godot_api]
impl INode for MainNode {
    fn ready(&mut self) {
        push_scene("res://scenes/Test.tscn");
    }
}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Global {
    base: Base<Node>,
}

#[godot_api]
impl INode for Global {
    fn ready(&mut self) {
        #[cfg(debug_assertions)]
        {
            godot_print!("Rust running in debug")
        }
        #[cfg(not(debug_assertions))]
        {
            godot_print!("Rust running in release")
        }
    }
}
