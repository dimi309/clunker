use gltf::Gltf;

#[derive(Default)]
pub struct Model {
	pub vertexData: Vec<f32>,
	pub indexData: Vec<u32>,
	pub textureCoordsData: Vec<f32>,
	pub buffers: Vec<gltf::buffer::Data>,
	pub images: Vec<gltf::image::Data>,
}


impl Model {

	fn parse_children(&mut self, ref node: gltf::Node) {
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
					let d = c.view().expect("Could not find view");

					let slice = &self.buffers[0][d.offset()..d.length()];
					let x = slice[0].clone();
				}
			}
			self.parse_children(child);
		}
	}

	pub fn load(&mut self) {

		let (document, buffers1, images1) = gltf::import("goat.glb").expect("Error while importing document, buffers and images");
		self.buffers = buffers1;
		self.images = images1;

		

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
                self.parse_children(node);
            }
        }
	}
}