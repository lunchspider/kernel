use x86_64::{
    structures::paging::{Page, PageTable, PageTableFlags, PhysFrame},
    PhysAddr, VirtAddr,
};

use crate::println;

pub const P4: *mut PageTable = 0xffffffff_fffff000 as *mut _;

pub struct RecursivePageTable {
    p4: *mut PageTable,
}

impl RecursivePageTable {
    pub fn new(p4_addr: VirtAddr) -> Self {
        Self {
            p4: p4_addr.as_u64() as *mut _,
        }
    }

    pub unsafe fn translate(&self, addr: VirtAddr) -> Option<PhysAddr> {
        let page = Page::containing_address(addr);

        self.translate_page(page)
            .map(|x| x.start_address() + addr.page_offset().into())
    }

    pub unsafe fn translate_page(&self, page: Page) -> Option<PhysFrame> {
        println!("{:?}", page);
        let p3 = &(&*self.p4)[page.p4_index()];

        println!("{:?}", p3);

        if p3.is_unused() {
            println!("3: we are here");
            return None;
        }

        let p2 = &(&*(p3.addr().as_u64() as *const PageTable))[page.p3_index()];

        println!("{:?}", p2);

        if p2.is_unused() {
            println!("2: we are here");
            return None;
        }

        let p1 = &(&*(p2.addr().as_u64() as *const PageTable))[page.p2_index()];

        println!("{:?}", p1);

        if p1.is_unused() {
            println!("1: we are here");
            return None;
        }

        println!("p1_index: {:?}", page.p1_index());
        println!("p2 frame addr: {:?}", p1);

        if p1.flags().contains(PageTableFlags::HUGE_PAGE) {
            return Some(PhysFrame::containing_address(PhysAddr::new(
                p1.addr().as_u64() + u64::from(page.p1_index()) * 4096,
            )));
        }

        Some(
            (&*(p1.addr().as_u64() as *const PageTable))[page.p1_index()]
                .frame()
                .unwrap(),
        )

        //Some(*p1)
    }
}
