#![no_std]

#[cfg(not(feature = "vdso"))]
use core::{mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicPtr};

#[cfg(not(feature = "vdso"))]
use lazyinit::LazyInit;

use crate::{deque::LockFreeDeque, ipc_item::IPCItem, slot_array::SlotArray};

mod api;
mod deque;
mod ipc_item;
mod slot_array;

vdso_helper::use_mut_cfg! {}

#[cfg(feature = "vdso")]
vdso_helper::vvar_data! {
    queue_array: SlotArray<LockFreeDeque<IPCItem, QUEUE_LEN>, ARRAY_LEN>,
}

#[cfg(not(feature = "vdso"))]
static QUEUE_ARRAY_ADDR: LazyInit<usize> = LazyInit::new();

/// Set the address of the queue array.
///
/// # Safety
///
/// The address must refer to a `SlotArray` that is already initialized,
/// and be valid for the lifetime of the program.
///
/// Before calling other functions, `set_queue_array_addr` or `set_queue_array_addr_and_init`
/// must be called once and only once.
#[cfg(not(feature = "vdso"))]
pub unsafe fn set_queue_array_addr(addr: NonNull<()>) {
    QUEUE_ARRAY_ADDR.init_once(addr.as_ptr() as usize);
}

#[cfg(not(feature = "vdso"))]
/// Initialize the queue array at the given address.
///
/// # Safety
///
/// The address must be valid for the lifetime of the program.
///
/// Before calling other functions, `set_queue_array_addr` or `set_queue_array_addr_and_init`
/// must be called once and only once.
pub unsafe fn set_queue_array_addr_and_init(addr: NonNull<()>) {
    QUEUE_ARRAY_ADDR.init_once(addr.as_ptr() as usize);
    unsafe {
        ((*QUEUE_ARRAY_ADDR.get().unwrap()) as *mut ()
            as *mut SlotArray<LockFreeDeque<IPCItem, QUEUE_LEN>, ARRAY_LEN>)
            .write(SlotArray::new())
    };
}

pub(crate) fn get_queue_array() -> &'static SlotArray<LockFreeDeque<IPCItem, QUEUE_LEN>, ARRAY_LEN>
{
    #[cfg(feature = "vdso")]
    {
        vdso_helper::get_vvar_data! {
            queue_array
        }
    }
    #[cfg(not(feature = "vdso"))]
    {
        unsafe {
            &*((*QUEUE_ARRAY_ADDR.get().expect(
                "QUEUE_ARRAY_ADDR is not initialized. Please call `set_queue_array_addr` or `set_queue_array_addr_and_init` first.",
            )) as *const ()
                as *const SlotArray<LockFreeDeque<IPCItem, QUEUE_LEN>, ARRAY_LEN>)
        }
    }
}

#[cfg(test)]
mod test_mut_cfg {
    extern crate std;

    use super::*;
    use std::println;

    // run with `cargo test test_constants -- --nocapture`
    #[test]
    fn test_constants() {
        println!("QUEUE_LEN: {}", QUEUE_LEN);
        println!("ARRAY_LEN: {}", ARRAY_LEN);
        // println!("BOOL_TEST: {}", BOOL_TEST);
        // println!("EXPR_TEST: {}", EXPR_TEST);
        // println!("FLOAT_TEST: {}", FLOAT_TEST);
    }
}
