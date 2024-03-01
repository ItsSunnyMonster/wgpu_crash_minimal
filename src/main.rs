use std::sync::Arc;
use env_logger::Env;
use wgpu::{Backends, DeviceDescriptor, Features, Instance, Limits, RequestAdapterOptions};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Icon, WindowBuilder};
use log::info;
use winit::dpi::LogicalSize;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .build(&event_loop).unwrap());

    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::PRIMARY, // DX12 backends does not support surface
        //backends: Backends::DX12, // Works fine
        //backends: Backends::VULKAN, // Works fine
        ..Default::default()
    });

    //let adapter = &instance.enumerate_adapters(Backends::all())[0];
    let adapter = &instance.enumerate_adapters(Backends::DX12)[0];
    //let adapter = &instance.enumerate_adapters(Backends::VULKAN)[0];

    let surface = instance.create_surface(window.clone()).unwrap();

    info!("Selected Adapter: {:?}", adapter.get_info());

    let (device, queue) = adapter.request_device(
        &DeviceDescriptor {
            required_features: Features::empty(),
            required_limits: Limits::default(),
            label: None,
        },
        None,
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    assert!(!surface_caps.formats.is_empty(), "Surface not compatible");

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                _ => {}
            },
            _ => {}
        }
    }).unwrap();
}
