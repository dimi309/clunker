#[derive(Default)]
pub struct Model {
    pub vertexData: Vec<f32>,
    pub indexData: Vec<u16>,
}

impl Model {
    pub fn load(&mut self, filepath: &str) {
        let (document, buffers, _) =
            gltf::import(filepath).expect("Error while importing document, buffers and images");

        if document.meshes().count() > 0 {
            let a = document.meshes().nth(0).expect("Could not retrieve mesh.");
            let b: Vec<gltf::Primitive> = a.primitives().collect();

            if b.len() > 0 {
                let pos = b[0].get(&gltf::Semantic::Positions).expect("no positions");
                assert!(5126 == pos.data_type().as_gl_enum()); // float (4 bytes)
                let posView = pos.view().expect("Could not find positions view");

                let vertexSlice = &buffers[posView.buffer().index()]
                    [posView.offset()..posView.offset() + posView.length()];

                let vertexDataTmp: Vec<f32> = vertexSlice
                    .chunks_exact(4)
                    .map(TryInto::try_into)
                    .map(Result::unwrap)
                    .map(f32::from_le_bytes)
                    .collect();

                let mut counter = 0;

                for vt in vertexDataTmp {
                    counter = counter + 1;
                    if counter == 3 {
                        self.vertexData.push(vt + 0.5);
                        self.vertexData.push(1f32);
                        counter = 0;
                    } else {
                        self.vertexData.push(vt / 2.0);
                    }
                }

                let ind = &b[0].indices().expect("No indices index found");
                assert!(5123 == ind.data_type().as_gl_enum()); // unsigned short (2)

                let indexView = ind.view().expect("View not found");

                let indexSlice = &buffers[indexView.buffer().index()]
                    [indexView.offset()..indexView.offset() + indexView.length()];

                self.indexData = indexSlice
                    .chunks_exact(2)
                    .map(TryInto::try_into)
                    .map(Result::unwrap)
                    .map(u16::from_le_bytes)
                    .collect();
            }
        }
    }
}
