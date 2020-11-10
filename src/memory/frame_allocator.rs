use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{PhysFrame, FrameAllocator, Size4KiB};
use x86_64::PhysAddr;
use alloc::vec::Vec;

static FREE_FRAMES: spin::Mutex<Vec<PhysAddr>> = spin::Mutex::new(Vec::new());

/// Returns an iterator over the usable frames specified in the memory map.
fn usable_frames(memory_map: &'static MemoryMap) -> impl Iterator<Item=PhysFrame> {
    // get usable regions from memory map
    let regions = memory_map.iter();
    let usable_regions = regions
        .filter(|r| r.region_type == MemoryRegionType::Usable);
    // map each region to its address range
    let addr_ranges = usable_regions
        .map(|r| r.range.start_addr()..r.range.end_addr());
    // transform to an iterator of frame start addresses
    let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
    // create `PhysFrame` types from the start addresses
    frame_addresses
        .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
}

pub fn init_frames(allocator: BootInfoFrameAllocator) -> RecyclingFrameAllocator {
    FREE_FRAMES.lock().extend(usable_frames(allocator.memory_map).map(|f| f.start_address()));

    for _ in 0..allocator.next {
        FREE_FRAMES.lock().pop();
    }

    RecyclingFrameAllocator {}
}

pub struct RecyclingFrameAllocator {}

impl RecyclingFrameAllocator {}

unsafe impl FrameAllocator<Size4KiB> for RecyclingFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let mut frame: Option<PhysFrame<Size4KiB>> = None;
        let phys_addr = FREE_FRAMES.lock().pop();

        if phys_addr.is_some() {
            frame = Some(PhysFrame::containing_address(phys_addr.unwrap()));
        }

        frame
    }
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> BootInfoFrameAllocator {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = usable_frames(self.memory_map).nth(self.next);
        self.next += 1;
        frame
    }
}
