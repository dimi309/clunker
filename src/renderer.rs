// Don't go crazy with warnings about all the stuff imported from C
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(target_os = "linux")]
use winit::platform::x11::WindowExtX11;

#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;
use std::ptr::addr_of;

const NUM_FRAMES_IN_FLIGHT: usize = 3;
const MAX_OBJECTS_PER_FRAME: usize = 10;

static mut descriptor_set_layout: VkDescriptorSetLayout = std::ptr::null_mut();

static binding_desc: [VkVertexInputBindingDescription; 2] = [
    VkVertexInputBindingDescription {
        binding: 0,
        stride: 4u32 * (std::mem::size_of::<f32>() as u32),
        inputRate: 0,
    },
    VkVertexInputBindingDescription {
        binding: 1,
        stride: 3u32 * (std::mem::size_of::<f32>() as u32),
        inputRate: 0,
    },
];

static attrib_desc: [VkVertexInputAttributeDescription; 2] = [
    VkVertexInputAttributeDescription {
        binding: 0,
        location: 0,
        format: VkFormat_VK_FORMAT_R32G32B32A32_SFLOAT,
        offset: 0,
    },
    VkVertexInputAttributeDescription {
        binding: 1,
        location: 1,
        format: VkFormat_VK_FORMAT_R32G32B32_SFLOAT,
        offset: 0,
    },
];

static mut command_buffer: [VkCommandBuffer; NUM_FRAMES_IN_FLIGHT] = [std::ptr::null_mut(); 3];

unsafe extern "C" fn set_input_state_callback(
    inputStateCreateInfo: *mut VkPipelineVertexInputStateCreateInfo,
) -> i32 {
    println!("Input state callback called.");

    (*inputStateCreateInfo).vertexBindingDescriptionCount = 2;
    (*inputStateCreateInfo).vertexAttributeDescriptionCount = 2;
    (*inputStateCreateInfo).pVertexBindingDescriptions = addr_of!(binding_desc[0]);
    (*inputStateCreateInfo).pVertexAttributeDescriptions = addr_of!(attrib_desc[0]);
    1
}

unsafe extern "C" fn set_pipeline_layout_callback(
    pipelineLayoutCreateInfo: *mut VkPipelineLayoutCreateInfo,
) -> i32 {
    println!("Pipeline layout callback called.");
    (*pipelineLayoutCreateInfo).pSetLayouts = std::ptr::addr_of_mut!(descriptor_set_layout);
    (*pipelineLayoutCreateInfo).setLayoutCount = 1;
    1
}

#[derive(Default)]
/// Transformation for a model, passed to the vertex shader
struct UboTransformation {
    transformation: [f32; 16],
    offset: [f32; 3],
    padding: [f32; 13],
}

const UBO_BINDING: u32 = 0;

/// The renderer struct, used for rendering models
pub struct Renderer {
    name_str: CString,

    pipeline_index: u32,

    real_screen_width: u32,
    real_screen_height: u32,

    descriptor_pool: VkDescriptorPool,

    descriptor_set: [VkDescriptorSet; NUM_FRAMES_IN_FLIGHT],

    ubo_buffer: [VkBuffer; NUM_FRAMES_IN_FLIGHT],
    ubo_buffer_memory: [VkDeviceMemory; NUM_FRAMES_IN_FLIGHT],
    ubo: UboTransformation,
}

impl Renderer {
    #[cfg(target_os = "windows")]
    fn init_vulkan(&self, window: &winit::window::Window) {
        // Using the vulkan helper
        unsafe {
            let res = vh_create_instance_and_surface_win32(
                self.name_str.as_ptr(),
                window.hinstance() as *mut HINSTANCE__,
                window.hwnd() as *mut HWND__,
            );

            if res > 0 {
                println!("Vulkan instance and surface created.")
            } else {
                panic!("Vulkan instance and surface creation has failed.");
            }
        }
    }
    #[cfg(target_os = "linux")]
    fn init_vulkan(&self, window: &winit::window::Window) {
        // Using the vulkan helper

        let c = window.xcb_connection().unwrap();
        let winv: u32 = window.xlib_window().unwrap().try_into().unwrap();

        let w: *mut u32 = &mut winv.clone();

        unsafe {
            let res = vh_create_instance_and_surface_linux(
                self.name_str.as_ptr(),
                c as *mut xcb_connection_t,
                w,
            );

            if res > 0 {
                println!("Vulkan instance and surface created.")
            } else {
                panic!("Vulkan instance and surface creation has failed.");
            }
        }
    }

    /// Create a renderer instance
    pub fn new(name: &str, window: &winit::window::Window) -> Renderer {
        let mut myself = Self {
            name_str: CString::new(name).expect("CString::new failed"),

            pipeline_index: 100,

            real_screen_width: 1024,
            real_screen_height: 768,

            descriptor_pool: std::ptr::null_mut(),

            descriptor_set: [
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ],
            ubo_buffer: [
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ],
            ubo_buffer_memory: [
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ],
            ubo: { Default::default() },
        };

        let work_dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let vertex_sharder_path =
            CString::new(work_dir.clone() + "/resources/shaders/vertexShader.spv")
                .expect("CString::new failed");
        let fragment_shader_path = CString::new(work_dir + "/resources/shaders/fragmentShader.spv")
            .expect("CString::new failed");
        unsafe {
            Renderer::init_vulkan(&myself, &window);

            if vh_init(NUM_FRAMES_IN_FLIGHT as u32) != 1 {
                panic!("Could not initialise Vulkan.");
            }

            vh_set_width_height(1024, 768);

            vh_create_sync_objects();

            if vh_create_swapchain() != 1 {
                panic!("Failed to create Vulkan swapchain.");
            }

            myself.create_descriptor_pool();

            myself.allocate_descriptor_sets();

            let iscb = Option::Some(
                set_input_state_callback
                    as unsafe extern "C" fn(*mut VkPipelineVertexInputStateCreateInfo) -> i32,
            );
            let iscc = Option::Some(
                set_pipeline_layout_callback
                    as unsafe extern "C" fn(*mut VkPipelineLayoutCreateInfo) -> i32,
            );

            let pidx_ptr: *mut u32 = &mut myself.pipeline_index;

            vh_create_pipeline(
                vertex_sharder_path.as_ptr(),
                fragment_shader_path.as_ptr(),
                iscb,
                iscc,
                pidx_ptr,
            );

            for n in 0..NUM_FRAMES_IN_FLIGHT {
                (myself.ubo_buffer[n], myself.ubo_buffer_memory[n]) =
                    super::descriptor::create_ubo_buffer(&myself.ubo);
            }
        }
        myself
    }

    /// Render a model
    pub fn render(&mut self, m: &super::model::Model, offset: [f32; 3]) {
        let mut current_frame_index = 0;

        let mut image_index_not_needed = 0;

        self.ubo.offset = offset;

        unsafe {
            vh_acquire_next_image(
                self.pipeline_index,
                &mut image_index_not_needed,
                &mut current_frame_index,
            );
            vh_wait_gpu_cpu_fence(current_frame_index);

            super::descriptor::update_ubo_buffer(
                &self.ubo,
                self.ubo_buffer_memory[current_frame_index as usize],
            );

            let cb_ptr: *mut VkCommandBuffer = &mut command_buffer[current_frame_index as usize];

            vh_destroy_draw_command_buffer(cb_ptr);

            if vh_new_pipeline_state == 1 {
                self.update_descriptor_sets();
            }

            vh_begin_draw_command_buffer(cb_ptr);
            let cb_cptr: *const VkCommandBuffer = &command_buffer[current_frame_index as usize];
            vh_bind_pipeline_to_command_buffer(self.pipeline_index, cb_cptr);
            let binding: [VkDeviceSize; 2] = [0, 0];
            let vb: [VkBuffer; 2] = [m.vertex_buffer, m.normals_buffer];
            vkCmdBindVertexBuffers(*cb_ptr, 0, 2, &vb[0], &binding[0]);
            vkCmdBindIndexBuffer(*cb_ptr, m.index_buffer, 0, VkIndexType_VK_INDEX_TYPE_UINT16);
            vkCmdBindDescriptorSets(
                *cb_ptr,
                VkPipelineBindPoint_VK_PIPELINE_BIND_POINT_GRAPHICS,
                *vh_pipeline_layout.wrapping_add(self.pipeline_index as usize),
                0,
                1,
                &self.descriptor_set[current_frame_index as usize],
                0,
                std::ptr::null(),
            );

            vkCmdDrawIndexed(*cb_ptr, m.index_data_size, 1, 0, 0, 0);
            vh_end_draw_command_buffer(cb_ptr);

            if vh_draw(cb_ptr, 1) != 1 {
                panic!("vh_draw has failed!");
            }
            if vh_draw(cb_ptr, 0) != 1 {
                panic!("vh_draw has failed!");
            };

            vh_present_next_image();
        }
    }

    /// Shutdown the GPU driver
    pub fn shutdown(&mut self) {
        unsafe {
            vkDeviceWaitIdle(vh_logical_device);
            for idx in 0..NUM_FRAMES_IN_FLIGHT {
                let cb_p: *mut VkCommandBuffer = &mut command_buffer[idx];
                vh_destroy_draw_command_buffer(cb_p);
            }

            vh_destroy_pipeline(self.pipeline_index);
            self.destroy_descriptor_sets();
            vkDestroyDescriptorPool(vh_logical_device, self.descriptor_pool, std::ptr::null());

            for n in 0..NUM_FRAMES_IN_FLIGHT {
                vh_destroy_buffer(self.ubo_buffer[n], self.ubo_buffer_memory[n]);
            }

            vh_destroy_swapchain();
            vh_destroy_sync_objects();
            vkDestroySurfaceKHR(vh_instance, vh_surface, std::ptr::null_mut());
            vh_shutdown()
        };
    }

    /// Set the width and length of the rendering window in pixels. This can
    /// be used also during window resize events coming from the application
    /// window.
    pub fn set_width_height(&mut self, width: u32, height: u32) {
        unsafe {
            self.real_screen_width = width;
            self.real_screen_height = height;
            if self.real_screen_width == 0 {
                self.real_screen_width = 1
            };
            if self.real_screen_height == 0 {
                self.real_screen_height = 1
            };
            vh_set_width_height(self.real_screen_width, self.real_screen_height);
            vh_recreate_pipelines_and_swapchain();
        }
    }

    fn create_descriptor_pool(&mut self) {
        let descriptor_pool_sizes: [VkDescriptorPoolSize; NUM_FRAMES_IN_FLIGHT] = [
            VkDescriptorPoolSize {
                type_: VkDescriptorType_VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptorCount: 1,
            },
            VkDescriptorPoolSize {
                type_: VkDescriptorType_VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptorCount: 1,
            },
            VkDescriptorPoolSize {
                type_: VkDescriptorType_VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptorCount: 1,
            },
        ];

        let descriptor_pool_create_info: *const VkDescriptorPoolCreateInfo =
            &VkDescriptorPoolCreateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
                poolSizeCount: 3,
                pPoolSizes: &descriptor_pool_sizes[0],
                flags:
                    VkDescriptorPoolCreateFlagBits_VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT
                        as u32,
                maxSets: NUM_FRAMES_IN_FLIGHT as u32 * MAX_OBJECTS_PER_FRAME as u32,
                pNext: std::ptr::null(),
            };

        let dpptr: *const VkDescriptorPoolCreateInfo = descriptor_pool_create_info;
        let dptr: *mut VkDescriptorPool = &mut self.descriptor_pool;

        unsafe {
            if vkCreateDescriptorPool(vh_logical_device, dpptr, std::ptr::null(), dptr)
                != VkResult_VK_SUCCESS
            {
                panic!("Failed to create descriptor pool");
            }
        }
    }

    fn allocate_descriptor_sets(&mut self) {
        let dslb = VkDescriptorSetLayoutBinding {
            binding: UBO_BINDING,
            descriptorType: VkDescriptorType_VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            descriptorCount: 1,
            stageFlags: VkShaderStageFlagBits_VK_SHADER_STAGE_VERTEX_BIT as u32,
            pImmutableSamplers: std::ptr::null(),
        };

        let dslci = VkDescriptorSetLayoutCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            bindingCount: 1,
            pBindings: &dslb,
            pNext: std::ptr::null(),
            flags: 0,
        };

        unsafe {
            if vkCreateDescriptorSetLayout(
                vh_logical_device,
                &dslci,
                std::ptr::null(),
                std::ptr::addr_of_mut!(descriptor_set_layout),
            ) != VkResult_VK_SUCCESS
            {
                panic!("Failed to create descriptor set layout");
            }
        }

        unsafe {
            let dsai = VkDescriptorSetAllocateInfo {
                sType: VkStructureType_VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
                descriptorPool: self.descriptor_pool,
                descriptorSetCount: 1,
                pSetLayouts: std::ptr::addr_of_mut!(descriptor_set_layout),
                pNext: std::ptr::null(),
            };

            for n in 0..NUM_FRAMES_IN_FLIGHT {
                self.descriptor_set[n] = std::ptr::null_mut();

                if vkAllocateDescriptorSets(vh_logical_device, &dsai, &mut self.descriptor_set[n])
                    != VkResult_VK_SUCCESS
                {
                    panic!("Failed to allocate descriptor set");
                }
            }
        }
    }

    fn destroy_descriptor_sets(&mut self) {
        for n in 0..NUM_FRAMES_IN_FLIGHT {
            unsafe {
                if vkFreeDescriptorSets(
                    vh_logical_device,
                    self.descriptor_pool,
                    1,
                    &self.descriptor_set[n],
                ) != VkResult_VK_SUCCESS
                {
                    panic!("Failed to free descriptor set");
                }
                self.descriptor_set[n] = std::ptr::null_mut(); // cannot find VK_NULL_HANDLE in bindings...
            }
        }
    }

    fn update_descriptor_sets(&mut self) {
        for n in 0..NUM_FRAMES_IN_FLIGHT {
            let dbi = VkDescriptorBufferInfo {
                buffer: self.ubo_buffer[n],
                offset: 0,
                range: std::mem::size_of::<UboTransformation>() as u64,
            };

            let wds = VkWriteDescriptorSet {
                sType: VkStructureType_VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
                pNext: std::ptr::null(),
                dstSet: self.descriptor_set[n],
                dstBinding: UBO_BINDING,
                dstArrayElement: 0,
                descriptorCount: 1,
                descriptorType: VkDescriptorType_VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                pImageInfo: std::ptr::null(),
                pBufferInfo: &dbi,
                pTexelBufferView: std::ptr::null(),
            };

            unsafe {
                vkUpdateDescriptorSets(vh_logical_device, 1, &wds, 0, std::ptr::null());
            }
        }
    }
}
