pub mod buffers;
pub mod compute_pipeline;
pub mod render_pipeline;

use crate::gpu::buffers::Buffers;
use crate::gpu::compute_pipeline::ComputePipeline;
use crate::gpu::render_pipeline::RenderPipeline;
use crate::LOCAL_GROUPS_X;
use wghf::model::{DrawModel, Model};

pub struct GPUData {
    pub buffers: Buffers,
    pub model: Model,
    pub render_pipeline: RenderPipeline,
    pub compute_pipeline: ComputePipeline,
    pub currently_written_index: usize,
}

impl GPUData {
    pub fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        buffers: Buffers,
        model: Model,
        material_layout: &wgpu::BindGroupLayout,
        mesh_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            render_pipeline: RenderPipeline::new(
                sc_desc,
                device,
                &buffers,
                material_layout,
                mesh_layout,
            ),
            compute_pipeline: ComputePipeline::new(device, &buffers),
            currently_written_index: 0,
            buffers,
            model,
        }
    }

    pub fn update(&mut self, encoder: &mut wgpu::CommandEncoder) {
        self.currently_written_index =
            (self.currently_written_index + 1) % self.compute_pipeline.bind_groups.len();

        let mut cpass = encoder.begin_compute_pass();
        cpass.set_pipeline(&self.compute_pipeline.pipeline);
        cpass.set_bind_group(
            0,
            &self.compute_pipeline.bind_groups[self.currently_written_index],
            &[],
        );
        cpass.dispatch(
            (self.buffers.model_data[self.currently_written_index].len() as f32
                / LOCAL_GROUPS_X as f32)
                .ceil() as u32,
            1,
            1,
        );
    }

    pub fn render(&self, frame: &wgpu::SwapChainOutput, encoder: &mut wgpu::CommandEncoder) {
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.render_pipeline.depth_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    clear_stencil: 0,
                }),
            });

            rpass.set_pipeline(&self.render_pipeline.pipeline);
            rpass.set_bind_group(
                2,
                &self.render_pipeline.bind_groups[self.currently_written_index],
                &[],
            );
            rpass.draw_model_instanced(
                &self.model,
                0..self.buffers.model_data[self.currently_written_index].len() as u32,
            );
        }
    }
}
