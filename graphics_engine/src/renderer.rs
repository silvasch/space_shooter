use winit::window::Window;

pub(crate) struct Renderer {
    pub window: Window,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        Renderer {
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
