use anyhow::Result;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::Renderer;

pub struct GraphicsEngine {
    event_loop: EventLoop<()>,

    renderer: Renderer,
}

impl GraphicsEngine {
    pub async fn new() -> Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop)?;

        let renderer = Renderer::new(window).await?;

        Ok(GraphicsEngine {
            event_loop,
            renderer,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    window_id,
                    ref event,
                } if window_id == self.renderer.window().id() => {
                    if !self.renderer.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                self.renderer.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                self.renderer.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                },
                Event::RedrawRequested(window_id) if window_id == self.renderer.window().id() => {
                    match self.renderer.render() {
                        Ok(_) => {},
                        Err(wgpu::SurfaceError::Lost) => self.renderer.resize(self.renderer.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                },
                Event::MainEventsCleared => {
                    self.renderer.window().request_redraw();
                }
                _ => {}
            })
    }
}
