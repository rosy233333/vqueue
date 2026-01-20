use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::{ARRAY_LEN, IPCItem, LockFreeDeque, PerProcess, QUEUE_CAPACITY, SlotGuard, SlotRef};

use crate::get_queue_array;

#[unsafe(no_mangle)]
pub extern "C" fn register_process() -> Result<SlotRef<'static, PerProcess, ARRAY_LEN>, ()> {
    get_queue_array().push(PerProcess::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn deque_push(process_id: usize, item: IPCItem) -> Result<(), IPCItem> {
    let slot_ref: SlotRef<'_, PerProcess, ARRAY_LEN> = unsafe { SlotRef::from_id(process_id) };
    let res = slot_ref.deque.push_front(item);
    slot_ref.into_id(); // prevent drop
    res
}

// // Don't work because of lifetime issue
// #[unsafe(no_mangle)]
// pub extern "C" fn push_slot(queue_id: usize) -> Result<SlotGuard<'static, IPCItem>, ()> {
//     let slot_ref: SlotRef<'static, PerProcess, ARRAY_LEN> =
//         unsafe { SlotRef::from_id(queue_id) };
//     let res: Result<SlotGuard<'static, IPCItem>, ()> = slot_ref.push_slot_front();
//     slot_ref.into_id(); // prevent drop
//     res
// }

#[unsafe(no_mangle)]
pub extern "C" fn deque_pop(process_id: usize) -> Option<IPCItem> {
    let slot_ref: SlotRef<'_, PerProcess, ARRAY_LEN> = unsafe { SlotRef::from_id(process_id) };
    let res = slot_ref.deque.pop_back();
    slot_ref.into_id(); // prevent drop
    res
}

/// # Safety
///
/// The caller must ensure that the id is get from `SlotRef::into_id`.
///
/// one id can only be converted back to one `SlotRef`.
#[unsafe(no_mangle)]
pub extern "C" fn slotref_from_id(process_id: usize) -> SlotRef<'static, PerProcess, ARRAY_LEN> {
    unsafe { SlotRef::from_id(process_id) }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_pid(process_id: usize, pid: usize) {
    let slot_ref: SlotRef<'_, PerProcess, ARRAY_LEN> = unsafe { SlotRef::from_id(process_id) };
    slot_ref.pid.store(pid, Ordering::Release);
}

/// 添加从msg_type（调度器协程id）到ntf_id（通知源id）的映射
#[unsafe(no_mangle)]
pub extern "C" fn map_add_entry(
    process_id: usize,
    msg_type: usize,
    ntf_id: usize,
) -> Result<(), ()> {
    let slot_ref: SlotRef<'_, PerProcess, ARRAY_LEN> = unsafe { SlotRef::from_id(process_id) };
    let res = slot_ref.map.push((msg_type, ntf_id));
    res.map(|sref| {
        mem::forget(sref); // 保持引用计数
    })
}

/// 根据msg_type（调度器协程id）查找ntf_id（通知源id）
#[unsafe(no_mangle)]
pub extern "C" fn map_get_ntf_id(process_id: usize, msg_type: usize) -> Option<usize> {
    let slot_ref: SlotRef<'_, PerProcess, ARRAY_LEN> = unsafe { SlotRef::from_id(process_id) };
    for i in 0..ARRAY_LEN {
        if let Some(&(this_msg_type, this_ntf_id)) = slot_ref.map.get(i) {
            if this_msg_type == msg_type || this_msg_type == usize::MAX {
                return Some(this_ntf_id);
            }
        }
    }
    None
}

/// 删除从msg_type（调度器协程id）到ntf_id（通知源id）的映射
#[unsafe(no_mangle)]
pub extern "C" fn map_pop_ntf_id(process_id: usize, msg_type: usize) -> Option<usize> {
    let slot_ref: SlotRef<'_, PerProcess, ARRAY_LEN> = unsafe { SlotRef::from_id(process_id) };
    for i in 0..ARRAY_LEN {
        if let Some(&(this_msg_type, this_ntf_id)) = slot_ref.map.get(i) {
            if this_msg_type == msg_type {
                // 删除slot
                unsafe {
                    slot_ref.map.drop_slot(i);
                }
                return Some(this_ntf_id);
            }
        }
    }
    None
}
