use anyhow::Result;
use winit::{event::{Event, WindowEvent, ElementState, VirtualKeyCode, KeyboardInput}, event_loop::{EventLoop, ControlFlow}, window::WindowBuilder};

use crate::Renderer;

pub struct GraphicsEngine {
    event_loop: EventLoop<()>,

    renderer: Renderer,
}

impl GraphicsEngine {
    pub async fn new() -> Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop)?;

        let renderer = Renderer::new(window);

        Ok(GraphicsEngine {
            event_loop,
            renderer,
        })
    }

    pub fn run(self) -> Result<()> {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    window_id,
                    ref event,
                } if window_id == self.renderer.window().id() => match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {},
                }
                _ => {}
            })
    }
}
