#![no_std]
#![no_main]

use core::{arch::asm, hint::black_box, panic::PanicInfo, ptr};

static VALUES: [u64; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
static mut BSS_ACC: u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start() -> ! {
    // SAFETY: `VALUES` and `BSS_ACC` reside in valid guest RAM mapped by `memory.x`.
    let sum = unsafe {
        let len = black_box(VALUES.len());
        let mut idx = black_box(0usize);
        while idx < len {
            let val = ptr::read_volatile(&VALUES[idx]);
            let acc = ptr::read_volatile(&raw const BSS_ACC);
            ptr::write_volatile(&raw mut BSS_ACC, acc.wrapping_add(val));
            idx += 1;
        }
        ptr::read_volatile(&raw const BSS_ACC)
    };

    // SAFETY: `ecall` cleanly hands control back to the emulator with `a0 = sum`.
    unsafe {
        asm!(
            "ecall",
            in("a7") 93u64,
            in("a0") sum,
            options(nomem, nostack)
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
