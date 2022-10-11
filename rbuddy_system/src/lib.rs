#![no_std]
#[deny(missing_docs)]

use buddy_allocator::{BuddyAllocator, LinkedListBuddy, UsizeBuddy};
use core::{alloc::Layout,ptr::NonNull};
use core::panic::PanicInfo;

type Allocator<const N: usize> = BuddyAllocator<N, UsizeBuddy, LinkedListBuddy>;


///page
#[repr(C, align(4096))]
pub struct Page([u8; 4096]);

impl Page {
    const ZERO: Self = Self([0; 4096]);
}


/// 64 MiB
static mut MEMORY: [Page; 16384] = [Page::ZERO; 16384];
//#[deny(non_upper_case_globals)]
static mut ALLOCATOR:Allocator<8> = Allocator::<8>::new();


#[no_mangle]
pub extern "C" fn init(){

    let ptr = NonNull::new(unsafe { MEMORY.as_mut_ptr() }).unwrap();
    let len = core::mem::size_of_val(unsafe { &MEMORY });
    unsafe{
        ALLOCATOR.init(12, ptr);
    }

    unsafe { ALLOCATOR.transfer(ptr, len) };
    
    unsafe{
        assert_eq!(len, ALLOCATOR.capacity());
        assert_eq!(len, ALLOCATOR.free());
    }

}

#[no_mangle]
pub extern "C" fn alloc()->NonNull<Page>{
    unsafe{
        let (point,_size) = ALLOCATOR.allocate_type::<Page>().unwrap();
        return point;
    }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut NonNull<Page>){
    let layout = Layout::new::<Page>();
    unsafe{
        ALLOCATOR.deallocate(NonNull::new(ptr).unwrap(), layout.size());
    }
}

#[panic_handler]
fn panic(_info:&PanicInfo)->!{
        loop{}
}


