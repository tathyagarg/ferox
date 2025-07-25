use log::info;
use std::{sync::Arc, time::Instant};
use wgpu::Color;

use winit::{dpi::PhysicalPosition, keyboard::ModifiersState, window::Window};

#[derive(Debug)]
pub struct FrameCounter {
    last_instant: Instant,
    frame_count: u32,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let new = Instant::now();
        let elapsed_secs = (new - self.last_instant).as_secs_f32();

        if elapsed_secs > 1.0 {
            let fps = self.frame_count as f32 / elapsed_secs;

            info!("{fps} FPS");

            self.last_instant = new;
            self.frame_count = 0;
        }
    }
}

#[derive(Debug)]
pub struct State {
    window: Arc<Window>,
    modifiers_state: ModifiersState,

    cursor_position: Option<PhysicalPosition<f64>>,

    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    pub frame_counter: FrameCounter,

    color: Color,
}

fn queue_callback() {
    info!("Queue callback executed");
}

impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        queue.on_submitted_work_done(queue_callback);

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let state = State {
            window,
            cursor_position: None,
            modifiers_state: Default::default(),
            device,
            queue,
            size,
            surface,
            surface_format,
            color: Color::BLACK,
            frame_counter: FrameCounter::new(),
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }

    pub fn update_cursor_position(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = Some(position);
    }

    pub fn get_cursor_position(&self) -> Option<PhysicalPosition<f64>> {
        self.cursor_position
    }

    pub fn render(&mut self) {
        // Create texture view
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        // Renders a GREEN screen
        let mut encoder = self.device.create_command_encoder(&Default::default());

        let color = match self.color {
            Color::BLACK => Color::RED,
            Color::RED => Color::BLUE,
            Color::BLUE => Color::GREEN,
            _ => Color::BLACK,
        };

        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.color = color;

        // If you wanted to call any drawing commands, they would go here.

        // End the renderpass.
        drop(renderpass);

        // Submit the command in the queue to execute
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();

        surface_texture.present();
    }
}
