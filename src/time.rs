pub unsafe fn wait_ms(ms: u64) {
    let freq: u64;
    let count: u64;
    asm!(
        "mrs {}, cntfrq_el0",
        "mrs {}, cntpct_el0",
        out(reg) freq,
        out(reg) count,
    );

    let count_until = count + ((freq / 1000) * ms);
    loop {
        let count: u64;
        asm!("mrs {}, cntpct_el0", out(reg) count);
        if count >= count_until {
            break;
        }
    }
}

pub unsafe fn wait_cycles(mut n: u32) {
    if n != 0 {
        while n != 0 {
            n -= 1;
            asm!("nop");
        }
    }
}
