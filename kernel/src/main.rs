#![no_std]
#![no_main]

use core::arch::asm;

use limine::framebuffer::Framebuffer;
use limine::request::FramebufferRequest;
use limine::BaseRevision;

use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight};

#[repr(C)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    fn raw_int(self) -> u32 {
        unsafe {
            return core::mem::transmute(self);
        };
    }
}

#[used]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

fn put_pixel(x: usize, y: usize, color: Color, framebuffer: &Framebuffer) {
    // pitch is provided in bytes
    let pixel_offset = y * framebuffer.pitch() as usize + x * 4;

    unsafe {
        *(framebuffer.addr().add(pixel_offset) as *mut u32) = color.raw_int();
    }
}

fn put_char(x: usize, y: usize, c: char) {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let width = get_raster_width(FontWeight::Regular, RasterHeight::Size24);

            let char_raster =
                get_raster(c, FontWeight::Regular, RasterHeight::Size24).expect("unsupported char");

            for (row_i, row) in char_raster.raster().iter().enumerate() {
                for (col_i, pixel) in row.iter().enumerate() {
                    let color = Color {
                        red: *pixel,
                        green: *pixel,
                        blue: *pixel,
                        alpha: 255,
                    };

                    let pixel_x = x * width + col_i;
                    let pixel_y = y * RasterHeight::Size24.val() + row_i;

                    put_pixel(pixel_x, pixel_y, color, &framebuffer);
                }
            }
        }
    }
}

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    for (i, c) in "Hello World!".chars().enumerate() {
        put_char(i, 0, c);
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
