use crate::MAX_BOIDS;
use itertools::Itertools;
use wghf::wghf_buffer;
use wghf::wghf_buffer::prelude::*;

#[buffer_data]
pub struct ViewData {
    pub projection: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub camera_pos: [f32; 4],
}

#[buffer_data]
pub struct BoidData {
    pub position: [f32; 4],
    pub velocity: [f32; 4],
    pub model: [[f32; 4]; 4],
}

#[buffer_data]
pub struct ComputeData {
    pub number_of_boids: u32,
    pub dt: f32,
}

pub struct Buffers {
    pub view: Buffer<ViewData>,
    pub model_data: Vec<Buffer<BoidData>>,
    pub compute_data: Buffer<ComputeData>,
}

impl Buffers {
    pub fn new(
        device: &wgpu::Device,
        view: ViewData,
        boid_data: &[BoidData],
        compute_data: ComputeData,
    ) -> Self {
        Self {
            view: Buffer::new(
                device,
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                &[view],
                None,
            ),
            model_data: (0..2)
                .map(|_| {
                    Buffer::new(
                        device,
                        wgpu::BufferUsage::STORAGE
                            | wgpu::BufferUsage::STORAGE_READ
                            | wgpu::BufferUsage::COPY_DST,
                        boid_data,
                        Some(MAX_BOIDS as usize),
                    )
                })
                .collect_vec(),
            compute_data: Buffer::new(
                device,
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                &[compute_data],
                None,
            ),
        }
    }
}
