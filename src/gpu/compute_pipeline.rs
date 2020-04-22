use crate::gpu::buffers::Buffers;
use itertools::Itertools;

pub struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
    pub bind_groups: Vec<wgpu::BindGroup>,
}

impl ComputePipeline {
    pub fn new(device: &wgpu::Device, buffers: &Buffers) -> Self {
        let compute_shader_module =
            device.create_shader_module(vk_shader_macros::include_glsl!("shaders/compute.comp"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::COMPUTE,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: false,
                    },
                },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: &pipeline_layout,
            compute_stage: wgpu::ProgrammableStageDescriptor {
                module: &compute_shader_module,
                entry_point: "main",
            },
        });

        let size = buffers.model_data.len();

        let bind_groups = (0..size)
            .map(|i| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
                    bindings: &[
                        wgpu::Binding {
                            binding: 0,
                            resource: buffers.compute_data.create_binding_resource(),
                        },
                        wgpu::Binding {
                            binding: 1,
                            resource: buffers.model_data[i].create_binding_resource(),
                        },
                        wgpu::Binding {
                            binding: 2,
                            resource: buffers.model_data[(i + 1) % size].create_binding_resource(),
                        },
                    ],
                    label: None,
                })
            })
            .collect_vec();

        Self {
            pipeline,
            bind_groups,
        }
    }
}
