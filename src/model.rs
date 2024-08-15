#[derive(Default)]
pub struct Model {
    pub buffers: Vec<gltf::buffer::Data>,
    pub vertexData: Vec<f32>,
    pub indexData: Vec<u16>,
    pub normalsData: Vec<f32>
}

impl Model {
    fn readF32PrimitivesData(
        &mut self,
        primitives: &Vec<gltf::Primitive>,
        dataVariable: &mut Vec<f32>,
        semantic: gltf::Semantic,
    ) {
        let accessor = primitives[0]
            .get(&semantic)
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

        // Sort data, adding a 1.0f 4th (w) component
        for vt in vertexDataTmp {
            dataVariable.push(vt);
            counter = counter + 1;
            if counter == 3 {
                
                if semantic == gltf::Semantic::Positions {
                    dataVariable.push(1f32);
                }
                counter = 0;
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
        let mut readVertexData: Vec<f32> = Vec::<f32>::new();
        self.readF32PrimitivesData(&primitives, &mut readVertexData, gltf::Semantic::Positions);
        self.vertexData = readVertexData;

        let mut readNormalsData: Vec<f32> = Vec::<f32>::new();
        self.readF32PrimitivesData(&primitives, &mut readNormalsData, gltf::Semantic::Normals);
        self.normalsData = readNormalsData;

        self.readIndexData(&primitives);
    }
}
