use crate::color::Color;

#[derive (Clone, Copy)]
pub struct KernelOptions {
    pub shorten: bool
}

#[derive (Clone, Copy)]
pub struct Options {
    pub color: Color,
    pub bold: bool,
    pub kernel_options: KernelOptions
}
