#![no_std]
#![no_main]

mod bad_apple;
mod writer;
// mod vga_buf;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};

use {
    bootloader_api::info::FrameBufferInfo,
    core::{panic::PanicInfo, ptr::read_volatile},
};

fn sleep_pseudo_ms(ms: u64) {
    for i in 0..ms * 100_000 {
        let _ = unsafe { read_volatile(&i) };
    }
}

use {crate::writer::Writer, core::fmt::Write};

fn extract_frames(info: &'static mut BootInfo) -> (&mut [u8], FrameBufferInfo) {
    let buf = info.framebuffer.as_mut().unwrap();
    let info = buf.info();
    (buf.buffer_mut(), info)
}

pub const CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size *= 256;
    config
};

fn kernel_entry(info: &'static mut BootInfo) -> ! {
    let (buf, info) = extract_frames(info);
    let mut writer = Writer::new(buf, info);

    {
        // let height = writer.height();
        // let width = writer.width();
        // writer.x_offset = height - 100 * font::RASTER_HEIGHT.val();
        // writer.y_offset = height - 34 * font::RASTER_WIDTH;

        writer.x_offset = 200;
        writer.y_offset = 80;
        writer.clear();
    }

    for frame in bad_apple::APPLE {
        writeln!(writer, "{frame}").unwrap();
        writer.pseudo_clear();
        // sleep_pseudo_ms(70);
    }
    loop {}
}

entry_point!(kernel_entry, config = &CONFIG);

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
