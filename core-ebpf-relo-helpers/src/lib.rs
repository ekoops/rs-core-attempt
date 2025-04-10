#![no_std]
#![allow(clippy::len_without_is_empty)]

#[cfg(not(target_arch = "bpf"))]
fn bpf_probe_read<T>(ptr: *const T) -> Result<T, i64> {
    unsafe { Ok(core::ptr::read(ptr)) }
}

#[cfg(target_arch = "bpf")]
use aya_ebpf::helpers::bpf_probe_read;

pub mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/relo_helpers.rs"));
}
