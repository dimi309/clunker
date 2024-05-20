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
                    let pos = b[0].get(&gltf::Semantic::Positions).expect("no positions");
                    assert!(5126 == pos.data_type().as_gl_enum()); // float (4 bytes)
                    let posView = pos.view().expect("Could not find positions view");

                    let vertexSlice =
                        &self.buffers[posView.index()][posView.offset()..posView.offset() + posView.length()];

                    let vertexDataTmp : Vec<f32> = vertexSlice
                        .chunks_exact(4)
                        .map(TryInto::try_into)
                        .map(Result::unwrap)
                        .map(f32::from_le_bytes)
                        .collect();

                    let mut counter = 0;
                

                    for vt in vertexDataTmp {
                        self.vertexData.push(vt);
                        counter = counter + 1;
                        if counter == 3 {
                            self.vertexData.push(1f32);
                            counter = 0;
                        }
                    }

                    let ind = &b[0].indices().expect("No indices index found");
                    assert!(5123 == ind.data_type().as_gl_enum()); // unsigned short (2)

                    let indexView = ind.view().expect("View not found");

					let indexSlice = &self.buffers[posView.index()][indexView.offset()..indexView.offset() + indexView.length()];

					let uidx: Vec<u16> = indexSlice
                        .chunks_exact(2)
                        .map(TryInto::try_into)
                        .map(Result::unwrap)
                        .map(u16::from_le_bytes)
                        .collect();

                    self.indexData = uidx.into_iter().map(|x| x as u32).collect();
                }
            }
            self.parse_children(child);
        }
    }

    pub fn load(&mut self, filepath: &str) {
        let (document, buffers1, images1) =
            gltf::import(filepath).expect("Error while importing document, buffers and images");
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
