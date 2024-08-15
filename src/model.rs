#[derive(Default)]
pub struct Model {
    pub buffers: Vec<gltf::buffer::Data>,
    pub vertexData: Vec<f32>,
    pub indexData: Vec<u16>,
}

impl Model {
    fn readVertexData(&mut self, primitives: &Vec<gltf::Primitive>) {
        let accessor = primitives[0]
            .get(&gltf::Semantic::Positions)
            .expect("Could not get positions accessor.");
        assert!(accessor.data_type() == gltf::accessor::DataType::F32); // float (4 bytes)
        let posView = accessor.view().expect("Could not find positions view.");

        let vertexSlice = &self.buffers[posView.buffer().index()]
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
                self.vertexData.push(vt);
                self.vertexData.push(1f32);
                counter = 0;
            } else {
                self.vertexData.push(vt);
            }
        }
    }

    fn readIndexData(&mut self, primitives: &Vec<gltf::Primitive>) {
        let ind = &primitives[0].indices().expect("No indices index found");
        assert!(5123 == ind.data_type().as_gl_enum()); // unsigned short (2)

        let indexView = ind.view().expect("View not found");

        let indexSlice = &self.buffers[indexView.buffer().index()]
            [indexView.offset()..indexView.offset() + indexView.length()];

        self.indexData = indexSlice
            .chunks_exact(2)
            .map(TryInto::try_into)
            .map(Result::unwrap)
            .map(u16::from_le_bytes)
            .collect();
    }

    pub fn load(&mut self, filepath: &str) {
        let (document, buffers1, _) =
            gltf::import(filepath).expect("Error while importing document, buffers and images");

        self.buffers = buffers1;

        if document.meshes().count() == 0 {
            return;
        }
        let mesh = document.meshes().nth(0).expect("Could not retrieve mesh.");
        let primitives: Vec<gltf::Primitive> = mesh.primitives().collect();

        if primitives.len() == 0 {
            return;
        }
        
        self.readVertexData(&primitives);
        self.readIndexData(&primitives);
    }
}
