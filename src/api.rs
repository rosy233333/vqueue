use crate::{ARRAY_LEN, IPCItem, LockFreeDeque, QUEUE_CAPACITY, SlotGuard, SlotRef};

use crate::get_queue_array;

#[unsafe(no_mangle)]
pub extern "C" fn register_queue()
-> Result<SlotRef<'static, LockFreeDeque<IPCItem, QUEUE_CAPACITY>, ARRAY_LEN>, ()> {
    get_queue_array().push(LockFreeDeque::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn push(queue_id: usize, item: IPCItem) -> Result<(), IPCItem> {
    let slot_ref: SlotRef<'_, LockFreeDeque<IPCItem, QUEUE_CAPACITY>, ARRAY_LEN> =
        unsafe { SlotRef::from_id(queue_id) };
    let res = slot_ref.push_front(item);
    slot_ref.into_id(); // prevent drop
    res
}

// // Don't work because of lifetime issue
// #[unsafe(no_mangle)]
// pub extern "C" fn push_slot(queue_id: usize) -> Result<SlotGuard<'static, IPCItem>, ()> {
//     let slot_ref: SlotRef<'static, LockFreeDeque<IPCItem, QUEUE_CAPACITY>, ARRAY_LEN> =
//         unsafe { SlotRef::from_id(queue_id) };
//     let res: Result<SlotGuard<'static, IPCItem>, ()> = slot_ref.push_slot_front();
//     slot_ref.into_id(); // prevent drop
//     res
// }

#[unsafe(no_mangle)]
pub extern "C" fn pop(queue_id: usize) -> Option<IPCItem> {
    let slot_ref: SlotRef<'_, LockFreeDeque<IPCItem, QUEUE_CAPACITY>, ARRAY_LEN> =
        unsafe { SlotRef::from_id(queue_id) };
    let res = slot_ref.pop_back();
    slot_ref.into_id(); // prevent drop
    res
}
