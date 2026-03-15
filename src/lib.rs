#[allow(
    nonstandard_style,
    unsafe_op_in_unsafe_fn,
    unused,
    unnecessary_transmutes,
)]
mod hell_ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    /*#[repr(C)]
    pub struct EngineState {
        pub synth: *mut SurgeSynthesizer,
        pub storage: *mut SurgeStorage,
        pub layer: *mut std::ffi::c_void,   
    }*/

    unsafe extern "C" { 
        pub fn create_engine(sr: f32) -> *mut SurgeSynthesizer;
        pub fn destroy_engine(surge: *mut SurgeSynthesizer);
    }
}

pub struct SurgeSynthesizer {
    ptr: *mut hell_ffi::SurgeSynthesizer,
}

impl SurgeSynthesizer {
    pub fn new(sample_rate: f32) -> Self {
        unsafe {
            let ptr = hell_ffi::create_engine(sample_rate);
            assert!(!ptr.is_null(), "a surge burnt the bridge down (failed to create the engine).");
            Self { ptr }
        }
    }

    pub fn process(&mut self) {
        unsafe {
            (*self.ptr).process();
        }
    }

    pub fn play_note(
        &mut self,
        channel: i8,
        key: i8,
        velocity: i8,
        detune: i8,
        host_noteid: i32,
        force_scene: i32,
    ) {
        unsafe {
            (*self.ptr).playNote(
                channel,
                key,
                velocity,
                detune,
                host_noteid,
                force_scene,
            );
        }
    }

    pub fn get_samples(&self) -> [[f32; 32]; 2] {
        unsafe {
            (*self.ptr).output
        }
    }
}

impl Drop for SurgeSynthesizer {
    fn drop(&mut self) {
        unsafe {
            hell_ffi::destroy_engine(self.ptr);
        }
    }
}
