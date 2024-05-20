use gltf::Gltf;

#[derive(Default)]
pub struct Model {
	pub vertexData: Vec<f32>,
	pub indexData: Vec<u32>,
	pub textureCoordsData: Vec<f32>,
}


impl Model {

	fn parse_children(ref node: gltf::Node) {
		for child in node.children() {
			println!(
				"Node #{} has {} children, name {}",
				child.index(),
				child.children().count(),
				child.name().unwrap_or_default()
			);
	
			if child.mesh().is_some() {
				println!("It's a mesh!!");
				let a = child.mesh().unwrap();
				let b: Vec<gltf::Primitive> = a.primitives().collect();
			  
				if b.len() > 0 {
					let c = b[0].get(&gltf::Semantic::Positions).expect("no positions");
					println!("GL datatype {}", c.data_type().as_gl_enum());
					let d = c.view();
					
				}
			}
			Self::parse_children(child);
		}
	}

	pub fn load(&self) {

		let (document, buffers, images) = gltf::import("goat.glb").expect("Error while importing document, buffers and images");

        for a in document.meshes() {
            println!("Mesh: {}", a.name().expect("Could not find mesh name"));
        }

        for scene in document.scenes() {
            for node in scene.nodes() {
                println!(
                    "Node #{} has {} children, name {}",
                    node.index(),
                    node.children().count(),
                    node.name().unwrap_or_default()
                );
                Self::parse_children(node);
            }
        }
	}
}