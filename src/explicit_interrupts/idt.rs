use x86_64::instructions::segmentation;
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;
use lazy_static::lazy_static;
use bit_field::BitField;
use crate::{println, exit_qemu, QemuExitCode};
pub type HandlerFunc = extern "C" fn() -> !;

pub struct Idt([Entry; 16]);

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            pointer_low: pointer as u16,
            gdt_selector,
            options: EntryOptions::new(),
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            reserved: 0,
        }
    }

    fn missing() -> Self {
        Entry {
            pointer_low: 0,
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            options: EntryOptions::minimal(),
            pointer_middle: 0,
            pointer_high: 0,
            reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    
    fn minimal() -> Self {
        let mut options = 0;
        options.set_bits(9..=11, 0b111);
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..=14, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..=2, index);
        self
    }
}

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt.set_handler(3, breakpoint_handler);
        idt.set_handler(8, double_fault_handler);
        
        idt
    };
}

extern "C" fn double_fault_handler() -> ! {
    println!("EXCEPTION: Double Fault Handler");
    exit_qemu(QemuExitCode::Success);
    loop {}
}


extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

extern "C" fn breakpoint_handler() -> ! {
    println!("BREAKPOINT!");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn init() {
    IDT.load();
}

impl Idt {
    fn new() -> Self {
        Idt([Entry::missing(); 16])
    }

    pub fn load(&'static self) {
        use x86_64::instructions::tables::{DescriptorTablePointer, lidt};
        use x86_64::addr::VirtAddr;
        use core::mem::size_of;

        let ptr = DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };
        // register interrupt descriptor table with CPU
        unsafe { lidt(&ptr) };
    }

    fn set_handler(&mut self, entry: u8, handler: HandlerFunc) {
        self.0[entry as usize] = Entry::new(
            segmentation::CS::get_reg(),
            handler,
        );
        //&mut self.0[entry as usize].options
    }
}

// #[test_case]
fn test_divide_by_zero_exception() {
    use crate::divide_by_zero;
    divide_by_zero();
}