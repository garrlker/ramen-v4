use crate::{error::Error, platform};

pub struct WindowBuilder {
    _a: (),
}

impl WindowBuilder {
    const fn new() -> Self {
        Self {
            _a: (),
        }
    }

    pub fn build(&self) -> Result<Window, Error> {
        platform::imp::WindowImpl::new(self).map(|imp| Window { imp })
    }
}

pub struct Window {
    imp: platform::imp::WindowImpl,
}

impl Window {
    pub const fn builder() -> WindowBuilder {
        WindowBuilder::new()
    }
}
