use crate::io;

static mut KERNEL: Kernel = Kernel {
    stdio: io::KernelStdio::None,
    name: [0; 128],
    major: 0,
    minor: 0,
    patch: 0,
};

pub fn get_kernel_ref() -> &'static Kernel {
    unsafe { &KERNEL }
}

pub struct KernelBuilder {
    stdio: io::KernelStdio,
    name: [u8; 128],
    major: u32,
    minor: u32,
    patch: u32,
}
impl KernelBuilder {
    pub fn new() -> KernelBuilder {
        KernelBuilder {
            stdio: io::KernelStdio::None,
            name: [0; 128],
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
    pub fn with_stdio(mut self, m: io::uart::MiniUart) -> KernelBuilder {
        self.stdio = io::KernelStdio::MiniUart(m);
        self
    }
    //pub fn with_allocator(allocator : &'static dyn GlobalAlocator){
    //
    //}
    pub fn with_memory_menagment_unit() {
        unimplemented!();
    }
    pub fn with_random_number_generator() {
        unimplemented!();
    }
    pub fn of_name(mut self, s: &str) -> KernelBuilder {
        if s.len() > 0 {
            let (base, _) = s.char_indices().next().unwrap();
            for (i, c) in s.char_indices() {
                self.name[i - base] = c as u8;
            }
        }
        self
    }
    pub fn version(mut self, major: u32, minor: u32, patch: u32) -> KernelBuilder {
        self.major = major;
        self.minor = minor;
        self.patch = patch;
        self
    }
    pub fn build(self) -> &'static Kernel {
        unsafe {
            crate::kernel::KERNEL = Kernel {
                stdio: self.stdio,
                name: self.name,
                major: self.major,
                minor: self.minor,
                patch: self.patch,
            };
            &KERNEL
        }
    }
}
pub struct Kernel {
    stdio: io::KernelStdio,
    name: [u8; 128],
    major: u32,
    minor: u32,
    patch: u32,
}

impl Kernel {
    pub fn get_stdio(&self) -> io::KernelStdio {
        self.stdio
    }
    /* pub fn get_stdio(&self) -> & io::Stdio {
        & self.stdio
    }*/
    pub fn get_name(&self) -> &str {
        let mut index = 0;
        for i in 0..128 {
            if self.name[i] == 0 {
                index = i;
                break;
            }
        }
        core::str::from_utf8(&self.name[0..index]).unwrap()
    }
    pub fn get_version(&self) -> (u32, u32, u32) {
        (self.major, self.minor, self.patch)
    }
}
