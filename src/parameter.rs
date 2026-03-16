use crate::hell_ffi;

pub struct Parameter {
    pub(super) ptr: *mut hell_ffi::Parameter,
}

impl Parameter {
    pub fn new() -> Self {
        Self { ptr: unsafe { &mut hell_ffi::Parameter::new() } }
    }
}

impl Drop for Parameter {
    fn drop(&mut self) {
        unsafe { hell_ffi::destroy_parameter(self.ptr); }
    }
}
