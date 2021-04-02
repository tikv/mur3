use core::ffi::*;
use std::os::raw::c_int;

extern "C" {
    pub fn MurmurHash3_x86_32(key: *const c_void, len: c_int, seed: u32, out: *mut c_void);
    pub fn MurmurHash3_x86_128(key: *const c_void, len: c_int, seed: u32, out: *mut c_void);
    pub fn MurmurHash3_x64_128(key: *const c_void, len: c_int, seed: u32, out: *mut c_void);
}

pub fn hash32(bytes: &[u8], seed: u32) -> u32 {
    let mut output: u32 = 0;
    unsafe {
        MurmurHash3_x86_32(
            bytes.as_ptr() as _,
            bytes.len() as i32,
            seed,
            &mut output as *mut u32 as _,
        );
    }
    output
}

pub fn hash128_86(bytes: &[u8], seed: u32) -> (u64, u64) {
    let mut output: (u64, u64) = (0, 0);
    unsafe {
        MurmurHash3_x86_128(
            bytes.as_ptr() as _,
            bytes.len() as i32,
            seed,
            &mut output as *mut (u64, u64) as _,
        );
    }
    output
}

pub fn hash128_64(bytes: &[u8], seed: u32) -> (u64, u64) {
    let mut output: (u64, u64) = (0, 0);
    unsafe {
        MurmurHash3_x64_128(
            bytes.as_ptr() as _,
            bytes.len() as i32,
            seed,
            &mut output as *mut (u64, u64) as _,
        );
    }
    output
}
