use multiboot2::MemoryArea;
use x86_64::{structures::paging::PhysFrame, PhysAddr};

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame>;
    fn deallocate(&mut self, frame: PhysFrame);
}

#[derive(Debug)]
pub struct AreaFrameAllocator<'a> {
    next_free_frame: PhysFrame,

    curr_area: Option<&'a MemoryArea>,

    kernel_start: PhysFrame,
    kernel_end: PhysFrame,

    multiboot_start: PhysFrame,
    multiboot_end: PhysFrame,

    memory_tag_areas: &'a [MemoryArea],
}

impl<'a> AreaFrameAllocator<'a> {
    #[inline]
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        memory_tag_areas: &'a [MemoryArea],
    ) -> Self {
        let kernel_start = PhysFrame::containing_address(PhysAddr::new(kernel_start as u64));

        let kernel_end = PhysFrame::containing_address(PhysAddr::new(kernel_end as u64));

        let multiboot_start = PhysFrame::containing_address(PhysAddr::new(multiboot_start as u64));

        let multiboot_end = PhysFrame::containing_address(PhysAddr::new(multiboot_end as u64));

        let mut allocator = Self {
            next_free_frame: PhysFrame::from_start_address(PhysAddr::new(0)).expect("GG"),

            curr_area: None,
            kernel_start,
            kernel_end,
            multiboot_start,
            multiboot_end,
            memory_tag_areas,
        };

        allocator.choose_next_area();

        allocator
    }

    #[inline]
    fn choose_next_area(&mut self) {
        self.curr_area = self
            .memory_tag_areas
            .iter()
            .filter(|area| {
                let address = area.start_address() + area.size() - 1;
                address >= self.next_free_frame.start_address().as_u64()
            })
            .min_by_key(|x| x.start_address());

        if let Some(area) = self.curr_area {
            let start_frame =
                PhysFrame::from_start_address(PhysAddr::new(area.start_address())).unwrap();
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl<'a> FrameAllocator for AreaFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        if let Some(area) = self.curr_area {
            let frame = self.next_free_frame;
            //println!("xx: {} {}", area.start_address(), area.size());
            let curr_end_frame =
                PhysFrame::containing_address(PhysAddr::new(area.start_address() + area.size()));

            if self.next_free_frame > curr_end_frame {
                self.choose_next_area();
            } else if self.next_free_frame >= self.kernel_start
                && self.next_free_frame <= self.kernel_end
            {
                self.next_free_frame = PhysFrame::from_start_address(PhysAddr::from(
                    self.kernel_end.start_address() + self.kernel_end.size(),
                ))
                .unwrap();
            } else if self.next_free_frame >= self.multiboot_start
                && self.next_free_frame <= self.multiboot_end
            {
                self.next_free_frame = PhysFrame::from_start_address(PhysAddr::from(
                    self.multiboot_end.start_address() + self.kernel_end.size(),
                ))
                .unwrap();
            } else {
                self.next_free_frame = PhysFrame::from_start_address(PhysAddr::from(
                    frame.start_address() + frame.size(),
                ))
                .unwrap();

                return Some(frame);
            }

            self.allocate_frame()
        } else {
            None
        }
    }

    fn deallocate(&mut self, frame: PhysFrame) {
        todo!()
    }
}
