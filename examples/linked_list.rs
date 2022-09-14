//use buddy_allocator::{BuddyAllocator, LinkedListBuddy, UsizeBuddy};
// use std::{
//     alloc::Layout,
//     ptr::{null_mut, NonNull},
//     time::Instant,
// };

use super::alloc;


// type Allocator<const N: usize> = BuddyAllocator<N, UsizeBuddy, LinkedListBuddy>;

// #[repr(C, align(4096))]
// struct Page([u8; 4096]);

// impl Page {
//     const ZERO: Self = Self([0; 4096]);
// }

// /// 64 MiB
// static mut MEMORY: [Page; 16384] = [Page::ZERO; 16384];

fn main() {
    //alloc::alloc();

    
    println!("hw linked test");
    let ptr= alloc::alloc();
    println!("hw test1");
    println!("ptr: {:?}  ",ptr);




//     let mut allocator = Allocator::<8>::new();
//     let ptr = NonNull::new(unsafe { MEMORY.as_mut_ptr() }).unwrap();
//     let len = core::mem::size_of_val(unsafe { &MEMORY });
//     allocator.init(12, ptr);
//     println!(
//         "MEMORY: {:#x}..{:#x}",
//         ptr.as_ptr() as usize,
//         ptr.as_ptr() as usize + len
//     );
//     let t = Instant::now();
//     unsafe { allocator.transfer(ptr, len) };
//     println!("transfer {:?}", t.elapsed());

//     assert_eq!(len, allocator.capacity());
//     assert_eq!(len, allocator.free());

//     println!(
//         "
// BEFORE
// {allocator:#x?}"
//     );

//     let mut blocks = [null_mut::<Page>(); 10000];
//     let layout = Layout::new::<Page>();
//     let t = Instant::now();

//     println!("hw allocate");
//     let mut i = 0;
//     for block in blocks.iter_mut() {
//         let (ptr, size) = allocator.allocate_type::<Page>().unwrap();
//         debug_assert_eq!(layout.size(), size);
//         *block = ptr.as_ptr();
//         i = i+1;
//     }
//     println!("hw i: {}",i);
//     let t = t.elapsed();
//     println!(
//         "allocate   {:?} ({} times)",
//         t / blocks.len() as u32,
//         blocks.len()
//     );

//     assert_eq!(len, allocator.capacity());
//     assert_eq!(len - blocks.len() * layout.size(), allocator.free());

//     let t = Instant::now();
//      println!("hw deallocate");
//     for block in blocks.iter_mut() {
//         //println!("hw test");
//         allocator.deallocate(NonNull::new(*block).unwrap(), layout.size());
//         *block = null_mut();
//     }
//     let t = t.elapsed();
//     println!(
//         "deallocate {:?} ({} times)",
//         t / blocks.len() as u32,
//         blocks.len()
//     );

//     assert_eq!(len, allocator.capacity());
//     assert_eq!(len, allocator.free());

//     println!(
//         "
// AFTER
// {allocator:#x?}"
//     );
}
