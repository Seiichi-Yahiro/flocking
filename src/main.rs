mod gpu;

use crate::gpu::buffers::{BoidData, Buffers, ComputeData, TimeData, ViewData};
use crate::gpu::GPUData;
use itertools::Itertools;
use wghf::model::{Material, Mesh, Model};
use wghf::wghf_buffer::prelude::*;
use wghf::wghf_camera::{Camera, CameraMode};
use wghf::wghf_window::prelude::*;
use wgpu::CommandEncoder;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;

pub const MAX_BOIDS: u32 = 1000;
pub const LOCAL_GROUPS_X: u32 = 32;

const RESOLUTION: [u32; 2] = [800, 600];
const FPS: u32 = 60;
const MAX_SKIP_FRAMES: u32 = 10;

pub struct State {
    pub projection: cgmath::Matrix4<f32>,
    pub camera: Camera,

    pub gpu_data: GPUData,
    pub time: f32,
}

impl State {
    pub fn resize(&mut self, device: &wgpu::Device, new_size: PhysicalSize<u32>) {
        self.projection = Self::calculate_projection(new_size);
        self.gpu_data.render_pipeline.depth_texture =
            wghf::texture::Texture::create_depth_texture(device, new_size.width, new_size.height);
    }

    pub fn calculate_projection(size: PhysicalSize<u32>) -> cgmath::Matrix4<f32> {
        cgmath::perspective(
            cgmath::Deg(90.0),
            (size.width as f32) / (size.height as f32),
            0.1,
            1000.0,
        )
    }
}

impl WindowState for State {
    fn init(init_data: InitData<'_>) -> (Self, Option<CommandEncoder>) {
        let mut encoder = init_data.device.create_default_encoder();

        let projection = Self::calculate_projection(init_data.window.inner_size());
        let camera = Camera::new(
            [0.0, 0.0, 10.0],
            CameraMode::Free {
                look_dir: cgmath::Vector3::from([0.0, 0.0, -1.0]),
                controls: Default::default(),
                camera_speed: 0.005,
                move_speed: 10.0,
            },
            /*CameraMode::Focused {
                look_at: cgmath::Point3::new(0.0, 0.0, 0.0),
                controls: Default::default(),
                speed: 0.005,
                zoom_speed: 0.01,
                wheel_zoom_speed: 0.01,
            },*/
        );

        let material_bind_group_layout = init_data
            .device
            .create_bind_group_layout(&Material::create_bind_group_layout_descriptor());

        let mesh_bind_group_layout = init_data
            .device
            .create_bind_group_layout(&Mesh::create_bind_group_layout_descriptor());

        let model = Model::load(
            init_data.device,
            &mut encoder,
            &material_bind_group_layout,
            &mesh_bind_group_layout,
            "models/guppy/guppy.obj",
        )
        .unwrap();

        let boids = (0..MAX_BOIDS)
            .map(|_| BoidData::new_random(-20.0..=20.0, -5.0..=5.0))
            .collect_vec();

        let buffers = Buffers::new(
            &init_data.device,
            ViewData {
                projection: projection.into(),
                view: camera.as_matrix().into(),
                camera_pos: camera.position.to_homogeneous().into(),
            },
            &boids,
            ComputeData {
                number_of_boids: boids.len() as u32,
                dt: init_data.dt as f32,
            },
        );

        let gpu_data = GPUData::init(
            &init_data.sc_desc,
            &init_data.device,
            buffers,
            model,
            &material_bind_group_layout,
            &mesh_bind_group_layout,
        );

        let state = Self {
            projection,
            camera,
            gpu_data,
            time: 0.0,
        };

        (state, Some(encoder))
    }

    fn event(&mut self, event_data: EventData<'_>) -> Option<CommandEncoder> {
        self.camera.event(event_data.event);

        if let Event::WindowEvent { event, .. } = event_data.event {
            match event {
                WindowEvent::Resized(physical_size) => {
                    self.resize(event_data.device, *physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    self.resize(event_data.device, **new_inner_size);
                }
                _ => {}
            }
        }

        event_data
            .event
            .on_key_press(|key| match key {
                VirtualKeyCode::Escape => {
                    *event_data.control_flow = ControlFlow::Exit;
                    None
                }
                _ => None,
            })
            .flatten()
    }

    fn update(&mut self, update_data: UpdateData<'_>) {
        self.camera.update(update_data.dt as f32);
        self.time += update_data.dt as f32;
        self.gpu_data
            .buffers
            .time_data
            .write_range(&[TimeData { time: self.time }], 0..1)
            .unwrap()
            .submit(update_data.device, update_data.encoder);
        self.gpu_data.update(update_data.encoder);
    }

    fn render(&mut self, render_data: RenderData<'_>) {
        self.gpu_data
            .buffers
            .view
            .write_range(
                &[ViewData {
                    projection: self.projection.into(),
                    view: self.camera.as_matrix().into(),
                    camera_pos: self.camera.position.to_homogeneous().into(),
                }],
                0..1,
            )
            .unwrap()
            .submit(render_data.device, render_data.encoder);

        self.gpu_data.render(render_data.frame, render_data.encoder);
    }
}

fn main() {
    futures::executor::block_on(Window::<State>::new(
        WindowBuilder::new()
            .with_title("Flocking")
            .with_resizable(false)
            .with_min_inner_size(LogicalSize::<u32>::from(RESOLUTION)),
        FPS,
        MAX_SKIP_FRAMES,
    ))
    .run();
}
