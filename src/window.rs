use crate::{error::Error, platform};
use std::borrow::Cow;

pub struct WindowBuilder {
    pub(crate) class_name: Cow<'static, str>,
    pub(crate) title: Cow<'static, str>,

    #[cfg(target_os = "windows")]
    pub(crate) cs_owndc: bool,
}

impl WindowBuilder {
    const fn new() -> Self {
        Self {
            class_name: Cow::Borrowed("ramen_window_class"),
            title: Cow::Borrowed("a nice window"),

            #[cfg(target_os = "windows")]
            cs_owndc: true,
        }
    }

    pub fn build(&self) -> Result<Window, Error> {
        platform::imp::WindowImpl::new(self).map(|imp| Window { imp })
    }

    pub fn class_name(&mut self, class_name: impl Into<Cow<'static, str>>) -> &mut Self {
        self.class_name = class_name.into();
        self
    }

    pub fn title(&mut self, title: impl Into<Cow<'static, str>>) -> &mut Self {
        self.title = title.into();
        self
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
