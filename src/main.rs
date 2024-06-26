mod common;

use winit::event_loop::EventLoop;
use std::borrow::Cow;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut primitive_type = "triangle-list";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        primitive_type = &args[1];
    }

    let mut topology = wgpu::PrimitiveTopology::TriangleList;
    let mut index_format = None;

    // let mut topology: wgpu::PrimitiveTopology = wgpu::PrimitiveTopology::PointList;
    // let mut index_format: Option<wgpu::IndexFormat> = None;


    // if primitive_type == "triangle-list" {
    //     topology = wgpu::PrimitiveTopology::TriangleList;
    //     index_format = Some(wgpu::IndexFormat::Uint32);
    // }
    if primitive_type == "triangle-strip" {
        topology = wgpu::PrimitiveTopology::TriangleStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    }
    else if primitive_type == "point-list" {
        topology = wgpu::PrimitiveTopology::PointList;
        index_format = None;
    } else if primitive_type == "line-list" {
        topology = wgpu::PrimitiveTopology::LineList;
        index_format = None;
    } else if primitive_type == "line-strip" {
        topology = wgpu::PrimitiveTopology::LineStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    }

    let inputs = common::Inputs{
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        topology,
        strip_index_format: index_format,
    };

    let event_loop = EventLoop::new()?;
    let window = winit::window::Window::new(&event_loop)?;
    window.set_title(&*format!("Hello, {}!", primitive_type));

    env_logger::init();

    pollster::block_on(common::run(event_loop, window, inputs, 9))?;

    Ok(())
}

