pub struct WindowBuilder {
    _a: (),
}

impl WindowBuilder {
    const fn new() -> Self {
        Self {
            _a: (),
        }
    }

    pub fn build() -> Window {
        Window::build()
    }
}

pub struct Window {
    _a: (),
}

impl Window {
    pub const fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }

    fn build() -> Self {
        todo!()
    }
}
