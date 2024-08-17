// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::renderer::*;

/// The model struct
pub struct Model {
    /// Data buffers read from gltf file
    pub buffers: Vec<gltf::buffer::Data>,
    /// Vertex data read from gltf file
    pub vertex_data: Vec<f32>,
    /// Index data read from gltf file
    pub index_data: Vec<u16>,
    /// Normals data read from gltf file
    pub normals_data: Vec<f32>,
    
    /// Vertex buffer on the GPU
    pub vertex_buffer: super::renderer::VkBuffer,
    /// Vertex buffer memory on the GPU
    pub vertex_buffer_memory: super::renderer::VkDeviceMemory,
    /// Index buffer on the GPU
    pub index_buffer: super::renderer::VkBuffer,
    /// Index buffer memory on the GPU
    pub index_buffer_memory: super::renderer::VkDeviceMemory,
    /// The size of the index data in bytes
    pub index_data_size: u32,
}

impl Model {
    /// Create a model
    pub fn new() -> Model {
        let myself = Model {
            buffers: Vec::<gltf::buffer::Data>::new(), 
            vertex_data: Vec::<f32>::new(), 
            index_data: Vec::<u16>::new(),
            normals_data: Vec::<f32>::new(),

            vertex_buffer: std::ptr::null_mut(),
            vertex_buffer_memory: std::ptr::null_mut(),
            index_buffer: std::ptr::null_mut(),
            index_buffer_memory: std::ptr::null_mut(),
            index_data_size: 0,
        };

        myself
    }

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

    /// Load model from a gltf (glb) file
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

    /// Create the GPU buffers and store the model on the GPU for later rendering
    pub fn to_gpu(&mut self) {
        let vertex_buffer_ptr = &mut self.vertex_buffer;
        let vertex_buffer_memory_ptr = &mut self.vertex_buffer_memory;

        let vertexDataSize = self.vertex_data.len();

        unsafe {
            if vh_create_buffer(
                vertex_buffer_ptr,
                (VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT
                    | VkBufferUsageFlagBits_VK_BUFFER_USAGE_VERTEX_BUFFER_BIT)
                    .try_into()
                    .unwrap(),
                (vertexDataSize * std::mem::size_of::<f32>())
                    .try_into()
                    .unwrap(),
                vertex_buffer_memory_ptr,
                VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT
                    .try_into()
                    .unwrap(),
            ) != 1
            {
                panic!("Failed to create postition buffer.");
            }
        }

        let mut staging_buffer: VkBuffer = std::ptr::null_mut();
        let mut staging_buffer_memory: VkDeviceMemory = std::ptr::null_mut();

        let staging_buffer_ptr = &mut staging_buffer;
        let staging_buffer_memory_ptr = &mut staging_buffer_memory;
        unsafe {
            if vh_create_buffer(
                staging_buffer_ptr,
                VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                    .try_into()
                    .unwrap(),
                (vertexDataSize * std::mem::size_of::<f32>())
                    .try_into()
                    .unwrap(),
                staging_buffer_memory_ptr,
                (VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                    | VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)
                    .try_into()
                    .unwrap(),
            ) != 1
            {
                panic!("Failed to create staging buffer for vertices.");
            }
        }
        let mut staging_data: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        let mut staging_data_ptr: *mut *mut ::std::os::raw::c_void = &mut staging_data;

        unsafe {
            vkMapMemory(
                vh_logical_device,
                staging_buffer_memory,
                0,
                VK_WHOLE_SIZE as u64,
                0,
                staging_data_ptr,
            );
        }

        let src_ptr = self.vertex_data.as_ptr() as *const f32;

        unsafe {
            std::ptr::copy_nonoverlapping(
                src_ptr as *const u8,
                staging_data as *mut u8,
                vertexDataSize * std::mem::size_of::<f32>(),
            );

            vkUnmapMemory(vh_logical_device, staging_buffer_memory);

            vh_copy_buffer(
                staging_buffer,
                self.vertex_buffer,
                (vertexDataSize * std::mem::size_of::<f32>())
                    .try_into()
                    .unwrap(),
            );

            vh_destroy_buffer(staging_buffer, staging_buffer_memory);
        }
        let index_buffer_ptr = &mut self.index_buffer;
        let index_buffer_memory_ptr = &mut self.index_buffer_memory;

        let indexDataSize = self.index_data.len();
        self.index_data_size = indexDataSize.try_into().unwrap();
        unsafe {
            if vh_create_buffer(
                index_buffer_ptr,
                (VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_DST_BIT
                    | VkBufferUsageFlagBits_VK_BUFFER_USAGE_INDEX_BUFFER_BIT)
                    .try_into()
                    .unwrap(),
                (indexDataSize * std::mem::size_of::<u16>())
                    .try_into()
                    .unwrap(),
                index_buffer_memory_ptr,
                VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT
                    .try_into()
                    .unwrap(),
            ) != 1
            {
                panic!("Failed to create index buffer");
            }
        }
        staging_buffer = std::ptr::null_mut();
        let staging_buffer_ptr = &mut staging_buffer;
        staging_buffer_memory = std::ptr::null_mut();
        let staging_buffer_memory_ptr = &mut staging_buffer_memory;
        unsafe {
            if vh_create_buffer(
                staging_buffer_ptr,
                VkBufferUsageFlagBits_VK_BUFFER_USAGE_TRANSFER_SRC_BIT
                    .try_into()
                    .unwrap(),
                (indexDataSize * std::mem::size_of::<u16>())
                    .try_into()
                    .unwrap(),
                staging_buffer_memory_ptr,
                (VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                    | VkMemoryPropertyFlagBits_VK_MEMORY_PROPERTY_HOST_COHERENT_BIT)
                    .try_into()
                    .unwrap(),
            ) != 1
            {
                panic!("Failed to create staging buffer for indices.");
            }
        }
        staging_data = std::ptr::null_mut();
        staging_data_ptr = &mut staging_data;
        unsafe {
            vkMapMemory(
                vh_logical_device,
                staging_buffer_memory,
                0,
                VK_WHOLE_SIZE as u64,
                0,
                staging_data_ptr,
            );
        }
        let src_ptr = self.index_data.as_ptr() as *const u32;

        unsafe {
            std::ptr::copy_nonoverlapping(
                src_ptr as *const u8,
                staging_data as *mut u8,
                (indexDataSize * std::mem::size_of::<u16>())
                    .try_into()
                    .unwrap(),
            );

            vkUnmapMemory(vh_logical_device, staging_buffer_memory);

            vh_copy_buffer(
                staging_buffer,
                self.index_buffer,
                (indexDataSize * std::mem::size_of::<u16>())
                    .try_into()
                    .unwrap(),
            );

            vh_destroy_buffer(staging_buffer, staging_buffer_memory);
        }
    }

    /// Clear the model from the GPU
    pub fn clear_gpu(&mut self) {
        unsafe {
            vh_destroy_buffer(self.vertex_buffer, self.vertex_buffer_memory);
            vh_destroy_buffer(self.index_buffer, self.index_buffer_memory);
        }

        self.vertex_buffer = std::ptr::null_mut();
        self.vertex_buffer_memory = std::ptr::null_mut();
        self.index_buffer = std::ptr::null_mut();
        self.index_buffer_memory = std::ptr::null_mut();
        self.index_data_size = 0;
    }
}
