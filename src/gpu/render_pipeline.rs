use crate::gpu::buffers::Buffers;
use itertools::Itertools;
use wghf::model::ModelVertexData;

pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_groups: Vec<wgpu::BindGroup>,
    pub depth_texture: wghf::texture::Texture,
}

impl RenderPipeline {
    pub fn new(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        buffers: &Buffers,
        material_layout: &wgpu::BindGroupLayout,
        mesh_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: true,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[material_layout, mesh_layout, &bind_group_layout],
        });

        let bind_groups = (0..buffers.model_data.len())
            .map(|i| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
                    bindings: &[
                        wgpu::Binding {
                            binding: 0,
                            resource: buffers.view.create_binding_resource(),
                        },
                        wgpu::Binding {
                            binding: 1,
                            resource: buffers.model_data[i].create_binding_resource(),
                        },
                        wgpu::Binding {
                            binding: 2,
                            resource: buffers.time_data.create_binding_resource(),
                        },
                    ],
                    label: None,
                })
            })
            .collect_vec();

        let vertex_shader =
            device.create_shader_module(vk_shader_macros::include_glsl!("shaders/render.vert"));
        let fragment_shader =
            device.create_shader_module(vk_shader_macros::include_glsl!("shaders/render.frag"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vertex_shader,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fragment_shader,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_read_mask: 0,
                stencil_write_mask: 0,
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[ModelVertexData::vertex_buffer_descriptor()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            pipeline,
            bind_groups,
            depth_texture: wghf::texture::Texture::create_depth_texture(
                device,
                sc_desc.width,
                sc_desc.height,
            ),
        }
    }
}
