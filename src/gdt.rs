use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // any other IST index would work too

// We use lazy_static because Rust's const evaluator is not yet
// powerful enough to do this initialization at compile time
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            // if not mut, STACK is mapped to a read-only page by the bootloader
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end // stacks on x86 grow downwards
        };
        tss
    };
}
