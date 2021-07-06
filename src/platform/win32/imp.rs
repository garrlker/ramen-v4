use crate::{error::Error, window::WindowBuilder};

pub(crate) struct WindowImpl {

}

impl WindowImpl {
    pub(crate) fn new(builder: &WindowBuilder) -> Result<Self, Error> {
        let _ = builder;
        todo!()
    }
}
