#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    hos::init();

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // uncomment line below to trigger a stack overflow
    // stack_overflow();

    // uncomment line below to trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // uncomment line below to invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // uncomment lines below to invoke a page fault
    // let ptr = 0x204df6 as *mut u32;
    // // read from a code page
    // unsafe {
    //     let x = *ptr;
    // }
    // println!("read worked");
    // // write to a code page
    // unsafe {
    //     *ptr = 42;
    // }
    // println!("write worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hos::test_panic_handler(info)
}
