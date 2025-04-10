#![no_std]
#![no_main]

use aya_ebpf::helpers::bpf_get_current_task;
use aya_ebpf::{macros::fexit, programs::FExitContext};
use aya_log_ebpf::info;

#[fexit]
fn core(ctx: FExitContext) -> u32 {
    let task = unsafe {bpf_get_current_task()} as *mut _;
    let start_time = unsafe {core_ebpf_relo_helpers::ffi::task_struct_start_time(task)};
    info!(&ctx, "task start time: {}", unsafe {*start_time});
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
