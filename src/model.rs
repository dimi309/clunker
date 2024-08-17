pub struct Model {
    pub buffers: Vec<gltf::buffer::Data>,
    pub vertex_data: Vec<f32>,
    pub index_data: Vec<u16>,
    pub normals_data: Vec<f32>,

    // Vulkan buffers
    pub vertex_buffer: super::renderer::VkBuffer,
    pub vertex_buffer_memory: super::renderer::VkDeviceMemory,

    pub index_buffer: super::renderer::VkBuffer,
    pub index_buffer_memory: super::renderer::VkDeviceMemory,
    pub index_data_size: u32,
}

impl Model {
    fn read_f32_primitives_data(
        &mut self,
        primitives: &Vec<gltf::Primitive>,
        data_variable: &mut Vec<f32>,
        semantic: gltf::Semantic,
    ) {
        let accessor = primitives[0]
            .get(&semantic)
            .expect("Could not get positions accessor.");
        assert!(accessor.data_type() == gltf::accessor::DataType::F32); // float (4 bytes)
        let pos_view = accessor.view().expect("Could not find positions view.");

        let vertex_slice = &self.buffers[pos_view.buffer().index()]
            [pos_view.offset()..pos_view.offset() + pos_view.length()];

        let vertex_data_tmp: Vec<f32> = vertex_slice
            .chunks_exact(4)
            .map(TryInto::try_into)
            .map(Result::unwrap)
            .map(f32::from_le_bytes)
            .collect();

        let mut counter = 0;

        // Sort data, adding a 1.0f 4th (w) component
        for vt in vertex_data_tmp {
            data_variable.push(vt);
            counter = counter + 1;
            if counter == 3 {
                if semantic == gltf::Semantic::Positions {
                    data_variable.push(1f32);
                }
                counter = 0;
            }
        }
    }

    fn read_index_data(&mut self, primitives: &Vec<gltf::Primitive>) {
        let ind = &primitives[0].indices().expect("No indices index found");
        assert!(5123 == ind.data_type().as_gl_enum()); // unsigned short (2)

        let index_view = ind.view().expect("View not found");

        let index_slice = &self.buffers[index_view.buffer().index()]
            [index_view.offset()..index_view.offset() + index_view.length()];

        self.index_data = index_slice
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
        let mut retrieved_vertex_data: Vec<f32> = Vec::<f32>::new();
        self.read_f32_primitives_data(
            &primitives,
            &mut retrieved_vertex_data,
            gltf::Semantic::Positions,
        );
        self.vertex_data = retrieved_vertex_data;

        let mut retrieved_normals_data: Vec<f32> = Vec::<f32>::new();
        self.read_f32_primitives_data(
            &primitives,
            &mut retrieved_normals_data,
            gltf::Semantic::Normals,
        );
        self.normals_data = retrieved_normals_data;

        self.read_index_data(&primitives);
    }
}
